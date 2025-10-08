<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Api;

use Svix\Models\EndpointHeadersOut;
use Svix\Models\HttpSinkHeadersPatchIn;
use Svix\Models\SinkTransformationOut;
use Svix\Request\SvixHttpClient;

class Stream
{
    public StreamEventType $eventType;
    public StreamEvents $events;
    public StreamSink $sink;
    public StreamStream $stream;

    public function __construct(
        private readonly SvixHttpClient $client,
    ) {
        $this->eventType = new StreamEventType($client);
        $this->events = new StreamEvents($client);
        $this->sink = new StreamSink($client);
        $this->stream = new StreamStream($client);
    }

    /** Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks. */
    public function sinkHeadersGet(
        string $streamId,
        string $sinkId,
    ): EndpointHeadersOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}/headers");
        $res = $this->client->send($request);

        return EndpointHeadersOut::fromJson($res);
    }

    /** Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks. */
    public function sinkHeadersPatch(
        string $streamId,
        string $sinkId,
        HttpSinkHeadersPatchIn $httpSinkHeadersPatchIn,
    ): EndpointHeadersOut {
        $request = $this->client->newReq('PATCH', "/api/v1/stream/{$streamId}/sink/{$sinkId}/headers");
        $request->setBody(json_encode($httpSinkHeadersPatchIn));
        $res = $this->client->send($request);

        return EndpointHeadersOut::fromJson($res);
    }

    /** Get the transformation code associated with this sink. */
    public function sinkTransformationGet(
        string $streamId,
        string $sinkId,
    ): SinkTransformationOut {
        $request = $this->client->newReq('GET', "/api/v1/stream/{$streamId}/sink/{$sinkId}/transformation");
        $res = $this->client->send($request);

        return SinkTransformationOut::fromJson($res);
    }
}
