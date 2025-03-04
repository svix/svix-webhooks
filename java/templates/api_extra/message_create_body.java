MessageInInternal msgInInternal = new MessageInInternal(messageIn);
if (msgInInternal.getTransformationsParams() != null) {
    if (msgInInternal.getTransformationsParams().get("rawPayload") == null) {
        // transformationsParams may be immutable
        HashMap<String, Object> trParams =
                new HashMap<>(msgInInternal.getTransformationsParams());
        trParams.put("rawPayload", msgInInternal.getPayload());
        msgInInternal.setTransformationsParams(trParams);
    }
} else {
    HashMap<String, Object> trParam = new HashMap<>();
    trParam.put("rawPayload", msgInInternal.getPayload());
    msgInInternal.setTransformationsParams(trParam);
}
msgInInternal.setPayload(new HashMap<>());
