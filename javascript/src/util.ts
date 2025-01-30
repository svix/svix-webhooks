export interface PostOptions {
  idempotencyKey?: string;
}

export class ApiException<T> extends Error {
  public headers: Record<string, string> = {};

  public constructor(
    public code: number,
    public body: T,
    headers: Headers
  ) {
    super(`HTTP-Code: ${code}\nHeaders: ${JSON.stringify(headers)}`);

    headers.forEach((value: string, name: string) => {
      this.headers[name] = value;
    });
  }
}
