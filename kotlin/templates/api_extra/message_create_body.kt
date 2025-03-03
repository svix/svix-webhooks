if (messageIn.transformationsParams != null) {
    // only set rawPayload if not already set
    if (messageIn.transformationsParams!!["rawPayload"] == null) {
        var trParams = (messageIn.transformationsParams as Map<String, Any>).toMutableMap();
        trParams["rawPayload"] = messageIn.payload
        messageIn.transformationsParams = trParams.toMap()
    }
} else {
    val trParams = mapOf("rawPayload" to messageIn.payload)
    messageIn.transformationsParams = trParams
}
messageIn.payload = ""
