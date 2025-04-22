MessageInInternal msgInInternal = new MessageInInternal(messageIn);
if (msgInInternal.transformationsParams != null) {
    if (msgInInternal.transformationsParams.get("rawPayload") == null) {
        // transformationsParams may be immutable
        HashMap<String, Object> trParams =
                new HashMap<>(msgInInternal.transformationsParams);
        trParams.put("rawPayload", msgInInternal.payload);
        msgInInternal.transformationsParams = trParams;
    }
} else {
    HashMap<String, Object> trParam = new HashMap<>();
    trParam.put("rawPayload", msgInInternal.payload);
    msgInInternal.transformationsParams = trParam;
}
msgInInternal.payload = new HashMap<>();
