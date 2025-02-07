using System;
using System.Collections.Generic;
using Microsoft.Extensions.Logging.Abstractions;
using Svix.Client;
using Svix.Model;
using Svix.Models;
using Xunit;

namespace Svix.Tests;

public class SvixClientTests
{
    [Fact]
    public void Constructor_WhenCalled_DoesNotNeedLogger()
    {
        var sut = new SvixClient("", new SvixOptions("http://some.url"));

        Assert.NotNull(sut);
    }

    [Fact]
    public void Constructor_WhenCalled_AcceptsLogger()
    {
        var sut = new SvixClient("", new SvixOptions("http://some.url"), new NullLogger<SvixClient>());

        Assert.NotNull(sut);
    }

    [IgnoreIfClientVarsUnset]
    public void KitchenSink_SeemsToWorkOkay()
    {
        var token = Environment.GetEnvironmentVariable("SVIX_TOKEN");
        var url = Environment.GetEnvironmentVariable("SVIX_SERVER_URL");
        var client = new SvixClient(token, new SvixOptions(url));

        var app = client.Application.Create(new ApplicationIn(name: "App"));
        try
        {
            client.EventType.Create(new EventTypeIn(name: "event.started", description: "Something started"));
        }
        catch (ApiException e)
        {
            // We expect conflicts, but any other status is an error
            Assert.Equal(409, e.ErrorCode);
        }

        try
        {
            client.EventType.Create(new EventTypeIn(name: "event.ended", description: "Something ended"));
        }
        catch (ApiException e)
        {
            // We expect conflicts, but any other status is an error
            Assert.Equal(409, e.ErrorCode);
        }

        var ep = client.Endpoint.Create(app.Id,
            new EndpointIn(url: "https://example.svix.com/", channels: new List<string> { "ch0", "ch1" }));

        ep.Channels.Sort();
        Assert.Equal(new List<string> { "ch0", "ch1" }, ep.Channels);
        Assert.Null(ep.FilterTypes);

        var epPatched = client.Endpoint.Patch(app.Id, ep.Id,
            new EndpointPatch(filterTypes: new List<string> { "event.started", "event.ended" }));
        epPatched.Channels.Sort();
        epPatched.FilterTypes.Sort();
        Assert.Equal(new List<string> { "ch0", "ch1" }, epPatched.Channels);
        Assert.Equal(new List<string> { "event.ended", "event.started" }, epPatched.FilterTypes);

        // Should not throw an exception if the serialization code handles empty bodies properly
        client.Endpoint.Delete(app.Id, ep.Id);
    }
}