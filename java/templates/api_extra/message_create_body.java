if (messageIn.getTransformationsParams() != null) {
    if (messageIn.getTransformationsParams().get("rawPayload") == null) {
        // transformationsParams may be immutable
        HashMap<String, Object> trParams = new HashMap<>(messageIn.getTransformationsParams());
        trParams.put("rawPayload",messageIn.getPayload());
        messageIn.setTransformationsParams(trParams);
    }
} else {
    HashMap<String, Object> trParam = new HashMap<>();
    trParam.put("rawPayload", messageIn.getPayload());
    messageIn.setTransformationsParams(trParam);
}
messageIn.setPayload("");
