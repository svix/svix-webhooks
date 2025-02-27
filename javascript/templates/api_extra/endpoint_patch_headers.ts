public headersPatch(
  appId: string,
  endpointId: string,
  endpointHeadersPatchIn: EndpointHeadersPatchIn
): Promise<void> {
  return this.patchHeaders(appId, endpointId, endpointHeadersPatchIn);
}
