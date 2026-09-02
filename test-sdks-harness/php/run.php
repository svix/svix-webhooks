<?php

declare(strict_types=1);

$repoRoot = dirname(__DIR__, 2);
$autoload = $repoRoot . '/vendor/autoload.php';
if (!is_file($autoload)) {
    fwrite(STDERR, "missing $autoload — composer install at the repo root\n");
    exit(1);
}
require $autoload;

use Svix\AutoConfig;
use Svix\AutoConfigConsumer;
use Svix\Models\EndpointIn;
use Svix\Models\SinkInCommon;

$fixturesPath = getenv('AUTOCONFIG_FIXTURES');
if ($fixturesPath === false || $fixturesPath === '') {
    fwrite(STDERR, "AUTOCONFIG_FIXTURES is unset\n");
    exit(1);
}

$fixtures = json_decode(file_get_contents($fixturesPath), true, 512, JSON_THROW_ON_ERROR);
$lang = $fixtures['languages']['php'];
$httpUrl = $fixtures['httpUrl'];
$eventType = $fixtures['eventType'];
$serverUrl = $fixtures['serverUrl'];
$orgToken = $fixtures['orgToken'];
$consumerId = $fixtures['consumerId'];
$appId = $lang['appId'];

$results = ['A' => null, 'B' => null, 'C' => null, 'D' => null, 'E' => null];
$notes = [];

function modelHasClass(string $repoRoot, string $rel): bool
{
    $path = $repoRoot . '/php/src/Models/' . $rel;
    $src = file_get_contents($path);

    return (bool) preg_match('/\bclass\s+\w+/', $src);
}

function runSubscribe(string $token, string $httpUrl, string $eventType): string
{
    try {
        $endpoint = EndpointIn::create($httpUrl)->withEventTypes([$eventType]);
        $out = (new AutoConfig($token, $endpoint))->subscribe();
        $id = $out->id ?? '';
        $url = $out->url ?? '';
        if ($id === '' || $url !== $httpUrl) {
            return sprintf(
                'FAIL — subscribe returned id=%s url=%s expected url=%s',
                $id === '' ? '(empty)' : $id,
                $url === '' ? '(empty)' : $url,
                $httpUrl
            );
        }

        return sprintf('PASS — subscribe id=%s url=%s', $id, $url);
    } catch (\Svix\Exception\ApiException $e) {
        $body = $e->getBody();
        $bodyStr = is_string($body) ? $body : json_encode($body);

        return sprintf('FAIL — HTTP %d body=%s', $e->getStatusCode(), $bodyStr);
    } catch (Throwable $e) {
        return sprintf('FAIL — %s: %s', $e::class, $e->getMessage());
    }
}

function sendMsg(string $serverUrl, string $orgToken, string $appId, string $eventType, string $src): void
{
    $ch = curl_init($serverUrl . '/api/v1/app/' . $appId . '/msg/');
    curl_setopt_array($ch, [
        CURLOPT_POST => true,
        CURLOPT_RETURNTRANSFER => true,
        CURLOPT_HTTPHEADER => [
            'Authorization: Bearer ' . $orgToken,
            'Content-Type: application/json',
        ],
        CURLOPT_POSTFIELDS => json_encode([
            'eventType' => $eventType,
            'payload' => ['ok' => true, 'src' => $src],
        ]),
    ]);
    $body = curl_exec($ch);
    $code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);
    if ($code < 200 || $code >= 300) {
        throw new RuntimeException("send msg HTTP $code $body");
    }
}

function pollOptions(): object
{
    return new \Svix\ApiInternal\MessagePollerv2ConsumerPollOptions(
        leaseDurationMs: 2000,
        startingPosition: \Svix\Models\StartingPosition::EARLIEST,
    );
}

function sinkInCommon(string $eventType): SinkInCommon
{
    return SinkInCommon::create()->withEventTypes([$eventType]);
}

function runPoller(
    string $token,
    string $src,
    string $eventType,
    string $consumerId,
    string $serverUrl,
    string $orgToken,
    string $appId,
): string {
    try {
        $consumer = new AutoConfigConsumer($token, sinkInCommon($eventType));
        $dest = $consumer->subscribe();
        if (($dest->id ?? '') === '') {
            return 'FAIL — subscribe returned empty dest id';
        }
        sendMsg($serverUrl, $orgToken, $appId, $eventType, $src);

        $opts = pollOptions();

        $match = null;
        for ($i = 0; $i < 10; $i++) {
            $poll = $consumer->receive($consumerId, $opts);
            foreach ($poll->data ?? [] as $msg) {
                $payload = $msg->payload ?? [];
                $gotSrc = is_array($payload) ? ($payload['src'] ?? null) : null;
                if (($msg->eventType ?? '') === $eventType && $gotSrc === $src) {
                    $match = $msg;
                    break 2;
                }
            }
            sleep(2);
        }
        if ($match === null) {
            return 'FAIL — dest=' . $dest->id . ' no matching message after 10 retries';
        }
        $offset = $match->offset;
        $consumer->commit($consumerId, $offset);
        $after = $consumer->receive($consumerId, $opts);
        foreach ($after->data ?? [] as $msg) {
            if (($msg->offset ?? null) === $offset) {
                return "FAIL — dest={$dest->id} offset={$offset} replayed after commit";
            }
        }

        return "PASS — dest={$dest->id} offset={$offset}";
    } catch (Throwable $e) {
        return sprintf('FAIL — %s: %s', $e::class, $e->getMessage());
    }
}

function runPollerExisting(
    string $token,
    string $src,
    string $eventType,
    string $consumerId,
    string $serverUrl,
    string $orgToken,
    string $appId,
): string {
    try {
        $binder = new AutoConfigConsumer($token, sinkInCommon($eventType));
        $dest = $binder->subscribe();
        if (($dest->id ?? '') === '') {
            return 'FAIL — subscribe returned empty dest id';
        }
        $consumer = new AutoConfigConsumer($token, sinkInCommon($eventType));
        sendMsg($serverUrl, $orgToken, $appId, $eventType, $src);

        $opts = pollOptions();

        $match = null;
        for ($i = 0; $i < 10; $i++) {
            $poll = $consumer->receive($consumerId, $opts);
            foreach ($poll->data ?? [] as $msg) {
                $payload = $msg->payload ?? [];
                $gotSrc = is_array($payload) ? ($payload['src'] ?? null) : null;
                if (($msg->eventType ?? '') === $eventType && $gotSrc === $src) {
                    $match = $msg;
                    break 2;
                }
            }
            sleep(2);
        }
        if ($match === null) {
            return 'FAIL — dest=' . $dest->id . ' no matching message after 10 retries';
        }
        $offset = $match->offset;
        $consumer->commit($consumerId, $offset);
        $after = $consumer->receive($consumerId, $opts);
        foreach ($after->data ?? [] as $msg) {
            if (($msg->offset ?? null) === $offset) {
                return "FAIL — dest={$dest->id} offset={$offset} replayed after commit";
            }
        }

        return "PASS — dest={$dest->id} offset={$offset} no subscribe on receiver";
    } catch (Throwable $e) {
        return sprintf('FAIL — %s: %s', $e::class, $e->getMessage());
    }
}

$results['A'] = runSubscribe($lang['v1Http'], $httpUrl, $eventType);
$results['B'] = runSubscribe($lang['v2Http'], $httpUrl, $eventType);

$blocked = [];
foreach (['DestinationIn.php', 'DestinationOut.php', 'AutoConfigSinkType.php'] as $file) {
    if (!modelHasClass($repoRoot, $file)) {
        $blocked[] = $file;
    }
}

if ($blocked !== []) {
    $msg = 'BLOCKED — generated stubs have no class: ' . implode(', ', $blocked);
    $results['C'] = $msg;
    $results['D'] = $msg;
    $results['E'] = $msg;
    $notes[] = 'Did not instantiate AutoConfigConsumer. Do not hand-fill @generated models.';
} else {
    $results['C'] = runPoller(
        $lang['v1Poller'],
        'php-v1',
        $eventType,
        $consumerId,
        $serverUrl,
        $orgToken,
        $appId,
    );
    $results['D'] = runPoller(
        $lang['v2Poller'],
        'php-v2',
        $eventType,
        $consumerId,
        $serverUrl,
        $orgToken,
        $appId,
    );
    $results['E'] = runPollerExisting(
        $lang['v2PollerExisting'],
        'php-e',
        $eventType,
        $consumerId,
        $serverUrl,
        $orgToken,
        $appId,
    );
}

echo "LANG: php\n";
echo "A: {$results['A']}\n";
echo "B: {$results['B']}\n";
echo "C: {$results['C']}\n";
echo "D: {$results['D']}\n";
echo "E: {$results['E']}\n";
echo 'Notes: ' . ($notes !== [] ? implode(' ', $notes) : 'none') . "\n";
