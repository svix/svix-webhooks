def message_in_raw(
    event_type: str, payload: str, content_type: t.Optional[str] = None
) -> MessageIn:
    """
    Creates a `MessageIn` with a raw string payload.

    The payload is not normalized on the server. Normally, payloads are required
    to be JSON, and Svix will minify the payload before sending the webhook
    (for example, by removing extraneous whitespace or unnecessarily escaped
    characters in strings). With this function, the payload will be sent
    "as is", without any minification or other processing.

    Args:
        event_type (str): The event type's name Example: `user.signup`.
        payload (str): Serialized message payload.
        content_type (str?): The value to use for the Content-Type header of the
            webhook sent by Svix, overwriting the default of `application/json`
            if specified.
    """
    transformations_params: t.Dict[str, t.Any] = {
        "rawPayload": payload,
    }
    if content_type is not None:
        transformations_params["headers"] = {"content-type": content_type}

    return MessageIn(
        event_type=event_type,
        payload={},
        transformations_params=transformations_params,
    )
