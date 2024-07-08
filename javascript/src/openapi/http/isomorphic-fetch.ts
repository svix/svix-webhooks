import { HttpLibrary, RequestContext, ResponseContext } from './http';
import { from, Observable } from '../rxjsStub';
import "svix-fetch";

const numRetries = 2;
const sleep = (interval: number) => new Promise(resolve => setTimeout(resolve, interval));

export class IsomorphicFetchHttpLibrary implements HttpLibrary {
  public send(request: RequestContext): Observable<ResponseContext> {
    const resultPromise = this.sendWithRetry(request, numRetries, 50, 1);
    return from<Promise<ResponseContext>>(resultPromise);
  }

  private async sendWithRetry(request: RequestContext, triesLeft: number, nextInterval: number, retryCount: number): Promise<ResponseContext> {
    try {
      const response = await this.sendOnce(request);
      if (triesLeft <= 0 || response.httpStatusCode < 500) {
        return response;
      }
    } catch (e) {
      if (triesLeft <= 0) {
        throw e;
      }
    };
    await sleep(nextInterval);
    const headers = request.getHeaders();
    headers['svix-retry-count'] = retryCount.toString()
    return await this.sendWithRetry(request, --triesLeft, nextInterval * 2, ++retryCount);
  }

  private sendOnce(request: RequestContext): Promise<ResponseContext> {
    let method = request.getHttpMethod().toString();
    let body = request.getBody();

    // Cloudflare Workers fail if the credentials option is used in a fetch call.
    // This work around that. Source:
    // https://github.com/cloudflare/workers-sdk/issues/2514#issuecomment-2178070014
    const isCredentialsSupported = "credentials" in Request.prototype;

    return fetch(request.getUrl(), {
      method: method,
      body: body as any,
      headers: request.getHeaders(),
      credentials: isCredentialsSupported ? "same-origin" : undefined
    }).then((resp: any) => {
      const headers: { [name: string]: string } = {};
      resp.headers.forEach((value: string, name: string) => {
        headers[name] = value;
      });

      const body = {
        text: () => resp.text(),
        binary: () => resp.blob()
      };
      return new ResponseContext(resp.status, headers, body);
    });
  }
}
