#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import time
import urllib.error
import urllib.request

from svix import AutoConfig, AutoConfigConsumer
from svix.api_internal.message_pollerv2 import MessagePollerv2ConsumerPollOptions
from svix.models import EndpointIn, SinkInCommon, StartingPosition

fixtures_path = os.environ["AUTOCONFIG_FIXTURES"]
with open(fixtures_path) as f:
    fixtures = json.load(f)

lang = fixtures["languages"]["python"]
server_url = fixtures["serverUrl"]
org_token = fixtures["orgToken"]
event_type = fixtures["eventType"]
http_url = fixtures["httpUrl"]
consumer_id = fixtures["consumerId"]
app_id = lang["appId"]
results: dict[str, tuple[str, str]] = {}


def rec(key: str, ok: bool, msg: str) -> None:
    results[key] = ("PASS" if ok else "FAIL", msg)


def send_msg(src: str) -> None:
    body = json.dumps(
        {"eventType": event_type, "payload": {"ok": True, "src": src}}
    ).encode()
    req = urllib.request.Request(
        f"{server_url}/api/v1/app/{app_id}/msg/",
        data=body,
        headers={
            "Authorization": f"Bearer {org_token}",
            "Content-Type": "application/json",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(req) as resp:
            resp.read()
    except urllib.error.HTTPError as e:
        raise RuntimeError(f"send msg HTTP {e.code}: {e.read().decode()}") from e


def poll_until(consumer: AutoConfigConsumer, src: str):
    opts = MessagePollerv2ConsumerPollOptions(
        starting_position=StartingPosition.EARLIEST,
        lease_duration_ms=2000,
    )
    last = None
    for _ in range(10):
        last = consumer.receive(consumer_id, opts)
        matches = [
            m
            for m in last.data
            if m.event_type == event_type and (m.payload or {}).get("src") == src
        ]
        if matches:
            return matches[0], last
        time.sleep(2)
    return None, last


def run_http(key: str, token: str) -> None:
    try:
        ep = AutoConfig(
            token, EndpointIn(url=http_url, event_types=[event_type])
        ).subscribe()
        if not ep.id:
            rec(key, False, "subscribe returned empty id")
        elif ep.url != http_url:
            rec(key, False, f"url mismatch: {ep.url!r}")
        else:
            rec(key, True, f"EndpointOut id={ep.id} url={ep.url}")
    except Exception as e:
        rec(key, False, f"{type(e).__name__}: {e}")


def run_consumer_existing(key: str, token: str, src: str) -> None:
    try:
        binder = AutoConfigConsumer(token, SinkInCommon(event_types=[event_type]))
        dest = binder.subscribe()
        if not dest.id:
            rec(key, False, f"subscribe returned empty id={dest.id!r}")
            return
        c = AutoConfigConsumer(token, SinkInCommon(event_types=[event_type]))
        send_msg(src)
        match, last = poll_until(c, src)
        if match is None:
            data_n = 0 if last is None else len(last.data)
            rec(key, False, f"no matching message after 10 retries dest={dest.id} last_n={data_n}")
            return
        offset = match.offset
        c.commit(consumer_id, offset)
        replay = c.receive(
            consumer_id,
            MessagePollerv2ConsumerPollOptions(
                starting_position=StartingPosition.EARLIEST,
                lease_duration_ms=2000,
            ),
        )
        if any(m.offset == offset for m in replay.data):
            rec(key, False, f"dest={dest.id} offset={offset} replayed after commit")
            return
        rec(key, True, f"dest={dest.id} offset={offset} src={src} no replay (no subscribe on receiver)")
    except Exception as e:
        extra = " DIOM_MISSING" if "501" in str(e) else ""
        rec(key, False, f"{type(e).__name__}: {e}{extra}")


def run_consumer(key: str, token: str, src: str) -> None:
    try:
        c = AutoConfigConsumer(token, SinkInCommon(event_types=[event_type]))
        dest = c.subscribe()
        if not dest.id:
            rec(key, False, f"subscribe returned empty id={dest.id!r}")
            return
        send_msg(src)
        match, last = poll_until(c, src)
        if match is None:
            data_n = 0 if last is None else len(last.data)
            rec(key, False, f"no matching message after 10 retries dest={dest.id} last_n={data_n}")
            return
        offset = match.offset
        c.commit(consumer_id, offset)
        replay = c.receive(
            consumer_id,
            MessagePollerv2ConsumerPollOptions(
                starting_position=StartingPosition.EARLIEST,
                lease_duration_ms=2000,
            ),
        )
        if any(m.offset == offset for m in replay.data):
            rec(key, False, f"dest={dest.id} offset={offset} replayed after commit")
            return
        rec(key, True, f"dest={dest.id} offset={offset} src={src} no replay")
    except Exception as e:
        extra = " DIOM_MISSING" if "501" in str(e) else ""
        rec(key, False, f"{type(e).__name__}: {e}{extra}")


run_http("A", lang["v1Http"])
run_http("B", lang["v2Http"])
run_consumer("C", lang["v1Poller"], "python-v1")
run_consumer("D", lang["v2Poller"], "python-v2")
run_consumer_existing("E", lang["v2PollerExisting"], "python-e")

print("LANG: python")
for k in ("A", "B", "C", "D", "E"):
    status, msg = results[k]
    print(f"{k}: {status} — {msg}")
print("Notes: fixtures.languages.python only; org token used only for POST /msg/")
