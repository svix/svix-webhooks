var msgInInternal =
    MessageInInternal(
        messageIn.application,
        messageIn.channels,
        messageIn.eventId,
        messageIn.eventType,
        mapOf(),
        messageIn.payloadRetentionHours,
        messageIn.payloadRetentionPeriod,
        messageIn.tags,
        messageIn.transformationsParams,
    )
if (msgInInternal.transformationsParams != null) {
    // only set rawPayload if not already set
    if (msgInInternal.transformationsParams!!["rawPayload"] == null) {
        var trParams =
            (msgInInternal.transformationsParams as Map<String, Any>).toMutableMap()
        trParams["rawPayload"] = messageIn.payload
        msgInInternal.transformationsParams = trParams.toMap()
    }
} else {
    val trParams = mapOf("rawPayload" to messageIn.payload)
    msgInInternal.transformationsParams = trParams
}
