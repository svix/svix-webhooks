import { Webhook, WebhookVerificationError } from './index';

test('missing id raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const payload = '{"test": 2432232314}';
    const headers = {
        "svix-timestamp": "1614265330",
        "svix-signature": "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
    };
    expect(() => {
        wh.verify(payload, headers);
    }).toThrowError(WebhookVerificationError);
});

test('missing timestamp raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const payload = '{"test": 2432232314}';
    const headers = {
        "svix-id": "msg_p5jXN8AQM9LWM0D4loKWxJek",
        "svix-signature": "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
    };
    expect(() => {
        wh.verify(payload, headers);
    }).toThrowError(WebhookVerificationError);});

test('missing signature raises error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const payload = '{"test": 2432232314}';
    const headers = {
        "svix-id": "msg_p5jXN8AQM9LWM0D4loKWxJek",
        "svix-timestamp": "1614265330",
    };
    expect(() => {
        wh.verify(payload, headers);
    }).toThrowError(WebhookVerificationError);});

test('invalid signature throws error', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const payload = '{"test": 2432232314}';
    const headers = {
        "svix-id": "msg_p5jXN8AQM9LWM0D4loKWxJek",
        "svix-timestamp": "1614265330",
        "svix-signature": "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVdawdawda"
    };
    expect(() => {
        wh.verify(payload, headers);
    }).toThrowError(WebhookVerificationError);
});

test('valid signature is valid and returns valid json', () => {
    const wh = new Webhook("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw");

    const payload = '{"test": 2432232314}';
    const headers = {
        "svix-id": "msg_p5jXN8AQM9LWM0D4loKWxJek",
        "svix-timestamp": "1614265330",
        "svix-signature": "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
    };
    wh.verify(payload, headers);
});
