using System.Net.Http.Headers;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Svix;
using Svix.ApiInternal;
using Svix.Models;

var fixturesPath = System.Environment.GetEnvironmentVariable("AUTOCONFIG_FIXTURES")
    ?? throw new InvalidOperationException("AUTOCONFIG_FIXTURES is unset");
var fixtures = JObject.Parse(File.ReadAllText(fixturesPath));
var lang = fixtures["languages"]!["csharp"]!;

var serverUrl = fixtures["serverUrl"]!.ToString();
var orgToken = fixtures["orgToken"]!.ToString();
var eventType = fixtures["eventType"]!.ToString();
var httpUrl = fixtures["httpUrl"]!.ToString();
var consumerId = fixtures["consumerId"]!.ToString();
var appId = lang["appId"]!.ToString();

Console.WriteLine("LANG: csharp");
Console.WriteLine($"A: {RunHttp(lang["v1Http"]!.ToString())}");
Console.WriteLine($"B: {RunHttp(lang["v2Http"]!.ToString())}");
Console.WriteLine($"C: {RunPoller(lang["v1Poller"]!.ToString(), "csharp-v1")}");
Console.WriteLine($"D: {RunPoller(lang["v2Poller"]!.ToString(), "csharp-v2")}");
Console.WriteLine($"E: {RunPollerExisting(lang["v2PollerExisting"]!.ToString(), "csharp-e")}");
Console.WriteLine("Notes: net8; ProjectReference csharp/Svix");

string RunHttp(string token)
{
    try
    {
        var ep = new AutoConfig(
            token,
            new EndpointIn
            {
                Url = httpUrl,
                EventTypes = new List<string> { eventType },
            }
        ).Subscribe();
        if (string.IsNullOrEmpty(ep.Id))
            return "FAIL — subscribe returned empty id";
        if (ep.Url != httpUrl)
            return $"FAIL — url mismatch got {ep.Url}";
        return $"PASS — EndpointOut id={ep.Id} url={ep.Url}";
    }
    catch (Exception e)
    {
        return $"FAIL — {OneLine(e)}";
    }
}

string RunPoller(string token, string src)
{
    try
    {
        var consumer = new AutoConfigConsumer(
            token,
            new SinkInCommon { EventTypes = new List<string> { eventType } }
        );
        var dest = consumer.Subscribe();
        if (string.IsNullOrEmpty(dest.Id))
            return "FAIL — subscribe returned empty dest id";
        if (dest.Config == null)
            return "FAIL — DestinationOut missing config";

        SendMsg(src);

        PollerV2MessageOut? found = null;
        string? lastErr = null;
        for (var i = 0; i < 10; i++)
        {
            try
            {
                var poll = consumer.Receive(
                    consumerId,
                    new MessagePollerv2ConsumerPollOptions
                    {
                        StartingPosition = StartingPosition.Earliest,
                        LeaseDurationMs = 2000,
                    }
                );
                found = poll.Data.FirstOrDefault(m =>
                    m.EventType == eventType && PayloadSrc(m.Payload) == src
                );
                if (found != null)
                    break;
                lastErr = $"empty/no match (n={poll.Data.Count})";
            }
            catch (Exception e)
            {
                if (OneLine(e).Contains("501"))
                    return $"FAIL — DIOM_MISSING dest={dest.Id} {OneLine(e)}";
                lastErr = OneLine(e);
            }
            Thread.Sleep(2000);
        }

        if (found == null)
            return $"FAIL — dest={dest.Id} no message after 10 retries last={lastErr}";

        var offset = found.Offset;
        consumer.Commit(consumerId, offset);

        var after = consumer.Receive(
            consumerId,
            new MessagePollerv2ConsumerPollOptions
            {
                StartingPosition = StartingPosition.Earliest,
                LeaseDurationMs = 2000,
            }
        );
        if (after.Data.Any(m => m.Offset == offset))
            return $"FAIL — dest={dest.Id} offset {offset} still present after commit";

        return $"PASS — dest={dest.Id} offset={offset}";
    }
    catch (Exception e)
    {
        return $"FAIL — {OneLine(e)}";
    }
}

string RunPollerExisting(string token, string src)
{
    try
    {
        var binder = new AutoConfigConsumer(
            token,
            new SinkInCommon { EventTypes = new List<string> { eventType } }
        );
        var dest = binder.Subscribe();
        if (string.IsNullOrEmpty(dest.Id))
            return "FAIL — subscribe returned empty dest id";

        var consumer = new AutoConfigConsumer(
            token,
            new SinkInCommon { EventTypes = new List<string> { eventType } }
        );

        SendMsg(src);

        PollerV2MessageOut? found = null;
        string? lastErr = null;
        for (var i = 0; i < 10; i++)
        {
            try
            {
                var poll = consumer.Receive(
                    consumerId,
                    new MessagePollerv2ConsumerPollOptions
                    {
                        StartingPosition = StartingPosition.Earliest,
                        LeaseDurationMs = 2000,
                    }
                );
                found = poll.Data.FirstOrDefault(m =>
                    m.EventType == eventType && PayloadSrc(m.Payload) == src
                );
                if (found != null)
                    break;
                lastErr = $"empty/no match (n={poll.Data.Count})";
            }
            catch (Exception e)
            {
                if (OneLine(e).Contains("501"))
                    return $"FAIL — DIOM_MISSING {OneLine(e)}";
                lastErr = OneLine(e);
            }
            Thread.Sleep(2000);
        }

        if (found == null)
            return $"FAIL — no message after 10 retries last={lastErr}";

        var offset = found.Offset;
        consumer.Commit(consumerId, offset);

        var after = consumer.Receive(
            consumerId,
            new MessagePollerv2ConsumerPollOptions
            {
                StartingPosition = StartingPosition.Earliest,
                LeaseDurationMs = 2000,
            }
        );
        if (after.Data.Any(m => m.Offset == offset))
            return $"FAIL — offset {offset} still present after commit";

        return $"PASS — dest={dest.Id} offset={offset} no subscribe on receiver";
    }
    catch (Exception e)
    {
        return $"FAIL — {OneLine(e)}";
    }
}

void SendMsg(string src)
{
    using var http = new HttpClient();
    http.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", orgToken);
    var body = JsonConvert.SerializeObject(
        new { eventType, payload = new { ok = true, src } }
    );
    var resp = http
        .PostAsync(
            $"{serverUrl}/api/v1/app/{appId}/msg/",
            new StringContent(body, Encoding.UTF8, "application/json")
        )
        .GetAwaiter()
        .GetResult();
    var respBody = resp.Content.ReadAsStringAsync().GetAwaiter().GetResult();
    if (!resp.IsSuccessStatusCode)
        throw new Exception($"send msg {src} HTTP {(int)resp.StatusCode}: {respBody}");
}

static string? PayloadSrc(object payload)
{
    if (payload is JObject jo)
        return jo["src"]?.ToString();
    return JToken.FromObject(payload)["src"]?.ToString();
}

static string OneLine(Exception e)
{
    var extra = "";
    if (e is ApiException api)
        extra = $" HTTP {api.ErrorCode} body={api.ErrorContent}";
    var msg = (e.GetType().Name + ": " + e.Message + extra).Replace('\n', ' ').Replace('\r', ' ');
    return msg.Length > 400 ? msg[..400] : msg;
}
