#!/usr/bin/env python3
"""Mint per-language AutoConfig tokens against a live Svix server.

Writes fixtures.json (gitignored). Tokens embed `surl` from the server; if that
host is unreachable (local nginx on :8040 is often 502), rewrite `surl` to
SVIX_SERVER_URL and re-encode with standard *padded* base64. Do not strip `=`.
"""

from __future__ import annotations

import base64
import json
import os
import sys
import time
import urllib.error
import urllib.request

LANGS = [
    "javascript",
    "python",
    "go",
    "rust",
    "java",
    "kotlin",
    "csharp",
    "ruby",
    "php",
]

EVENT_TYPE = "it.autoconfig.ping"
HTTP_URL = "https://example.svix.com/it-autoconfig"
CONSUMER_ID = "it-consumer"


def die(msg: str, code: int = 1) -> None:
    print(f"setup: {msg}", file=sys.stderr)
    raise SystemExit(code)


def api(method: str, url: str, token: str, body: dict | None = None) -> tuple[int, object]:
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(
        url,
        data=data,
        method=method,
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
            "Accept": "application/json",
        },
    )
    try:
        with urllib.request.urlopen(req) as resp:
            raw = resp.read()
            if not raw:
                return resp.status, None
            return resp.status, json.loads(raw.decode())
    except urllib.error.HTTPError as e:
        raw = e.read().decode()
        try:
            parsed = json.loads(raw) if raw else None
        except json.JSONDecodeError:
            parsed = raw
        return e.code, parsed


def decode_magic(token: str) -> dict:
    if token.startswith("auto_v1_"):
        payload = token[len("auto_v1_") :]
    elif token.startswith("auto_v2_"):
        payload = token[len("auto_v2_") :]
    else:
        raise ValueError(f"not a magic token: {token[:20]!r}")
    pad = "=" * ((4 - len(payload) % 4) % 4)
    return json.loads(base64.b64decode(payload + pad))


def rewrite_surl(token: str, surl: str) -> str:
    """Re-encode the magic token with a new surl. Keep STANDARD padding."""
    if token.startswith("auto_v1_"):
        prefix = "auto_v1_"
    elif token.startswith("auto_v2_"):
        prefix = "auto_v2_"
    else:
        raise ValueError(f"not a magic token: {token[:20]!r}")
    content = decode_magic(token)
    if content.get("surl") == surl:
        return token
    content["surl"] = surl
    raw = json.dumps(content, separators=(",", ":")).encode()
    return prefix + base64.b64encode(raw).decode("ascii")


def require_token_field(resp: object, path: str) -> str:
    if not isinstance(resp, dict) or "token" not in resp:
        raise RuntimeError(f"{path}: expected {{token}}, got {resp!r}")
    return str(resp["token"])


def main() -> None:
    harness = os.path.dirname(os.path.abspath(__file__))
    out_path = os.environ.get("AUTOCONFIG_FIXTURES", os.path.join(harness, "fixtures.json"))
    server = os.environ.get("SVIX_SERVER_URL", "http://localhost:8071").rstrip("/")
    org = os.environ.get("SVIX_TOKEN", "").strip()
    if not org:
        die("SVIX_TOKEN is unset (org / management token, not an auto_v* string)")

    code, _ = api("GET", f"{server}/api/v1/health", org)
    if code != 204:
        die(f"health {server}/api/v1/health returned {code}, expected 204")

    code, body = api(
        "POST",
        f"{server}/api/v1/event-type",
        org,
        {"name": EVENT_TYPE, "description": "autoconfig integration"},
    )
    if code not in (200, 201, 409):
        die(f"create event type HTTP {code}: {body}")

    stamp = int(time.time())
    smoke_app_name = f"it-ac-smoke-{stamp}"
    code, app = api("POST", f"{server}/api/v1/app/", org, {"name": smoke_app_name})
    if code not in (200, 201) or not isinstance(app, dict):
        die(f"smoke create app HTTP {code}: {app}")
    smoke_app = app["id"]

    code, created = api("POST", f"{server}/api/v1/app/{smoke_app}/autoconfig", org)
    if code not in (200, 201):
        die(f"smoke create v2 autoconfig HTTP {code}: {created}")
    magic = rewrite_surl(require_token_field(created, "smoke autoconfig"), server)
    decoded = decode_magic(magic)
    tok = decoded["tok"]
    sid = decoded["sid"]
    aid = decoded["aid"]

    code, dest = api(
        "PUT",
        f"{server}/api/v1/app/{aid}/autoconfig/{sid}/destination",
        tok,
        {"type": "pollingEndpoint", "eventTypes": [EVENT_TYPE]},
    )
    if code == 501:
        die("DIOM_MISSING: subscribe destination returned 501. Enable polling/Diom on the server.")
    if code not in (200, 201) or not isinstance(dest, dict):
        die(f"smoke subscribe destination HTTP {code}: {dest}")
    dest_id = dest["id"]

    code, _msg = api(
        "POST",
        f"{server}/api/v1/app/{aid}/msg/",
        org,
        {"eventType": EVENT_TYPE, "payload": {"ok": True, "src": "smoke"}},
    )
    if code not in (200, 201, 202):
        die(f"smoke send msg HTTP {code}: {_msg}")

    poll_url = (
        f"{server}/api/v1/app/{aid}/polling-endpoint/{dest_id}/consumer/smoke"
        "?starting_position=earliest&lease_duration_ms=2000"
    )
    ok = False
    last = None
    for _ in range(8):
        code, last = api("GET", poll_url, tok)
        if code == 501:
            die("DIOM_MISSING: poll returned 501. Enable polling/Diom on the server.")
        if code == 200 and isinstance(last, dict):
            data = last.get("data") or []
            if any(
                (m.get("eventType") == EVENT_TYPE)
                and isinstance(m.get("payload"), dict)
                and m["payload"].get("src") == "smoke"
                for m in data
            ):
                ok = True
                break
        time.sleep(2)
    if not ok:
        die(f"smoke poll never saw src=smoke (last HTTP {code}: {last})")

    print(f"setup: smoke ok dest={dest_id} app={smoke_app}", file=sys.stderr)

    languages: dict[str, dict[str, str]] = {}
    for lang in LANGS:
        code, app = api(
            "POST",
            f"{server}/api/v1/app/",
            org,
            {"name": f"it-ac-{lang}-{stamp}"},
        )
        if code not in (200, 201) or not isinstance(app, dict):
            die(f"{lang}: create app HTTP {code}: {app}")
        app_id = app["id"]

        tokens: dict[str, str] = {}
        for key, path in (
            ("v1Http", f"/api/v1/app/{app_id}/auto-config"),
            ("v1Poller", f"/api/v1/app/{app_id}/auto-config"),
            ("v2Http", f"/api/v1/app/{app_id}/autoconfig"),
            ("v2Poller", f"/api/v1/app/{app_id}/autoconfig"),
            ("v2PollerExisting", f"/api/v1/app/{app_id}/autoconfig"),
        ):
            code, created = api("POST", server + path, org)
            if code not in (200, 201):
                die(f"{lang} {key} HTTP {code}: {created}")
            tokens[key] = rewrite_surl(require_token_field(created, f"{lang} {key}"), server)

        languages[lang] = {"appId": app_id, **tokens}
        print(f"setup: {lang} app={app_id}", file=sys.stderr)

    fixtures = {
        "serverUrl": server,
        "orgToken": org,
        "eventType": EVENT_TYPE,
        "httpUrl": HTTP_URL,
        "consumerId": CONSUMER_ID,
        "surlNote": "surl rewritten to SVIX_SERVER_URL with STANDARD padded base64 (no strip)",
        "languages": languages,
    }
    os.makedirs(os.path.dirname(out_path) or ".", exist_ok=True)
    with open(out_path, "w") as f:
        json.dump(fixtures, f, indent=2)
        f.write("\n")
    print(f"setup: wrote {out_path}", file=sys.stderr)


if __name__ == "__main__":
    main()
