public headersUpdate(
  appId: string,
  endpointId: string,
  endpointHeadersIn: EndpointHeadersIn
): Promise<void> {
  return this.updateHeaders(appId, endpointId, endpointHeadersIn);
}
