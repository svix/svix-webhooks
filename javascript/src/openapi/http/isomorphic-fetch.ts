import {HttpLibrary, RequestContext, ResponseContext} from './http';
import { from, Observable } from '../rxjsStub';
import "isomorphic-fetch";

const numRetries = 2;
const sleep = (interval: number) => new Promise(resolve => setTimeout(resolve, interval));

export class IsomorphicFetchHttpLibrary implements HttpLibrary {

    public send(request: RequestContext): Observable<ResponseContext> {
      const resultPromise = this.sendWithRetry(request, numRetries, 50);
      return from<Promise<ResponseContext>>(resultPromise);
    }

    private async sendWithRetry(request: RequestContext, triesLeft: number, nextInterval: number): Promise<ResponseContext> {
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
      return await this.sendWithRetry(request, --triesLeft, nextInterval * 2);
    }
  
    private sendOnce(request: RequestContext): Promise<ResponseContext> {
        let method = request.getHttpMethod().toString();
        let body = request.getBody();

        return fetch(request.getUrl(), {
            method: method,
            body: body as any,
            headers: request.getHeaders(),
            credentials: "same-origin"
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
