/* istanbul ignore file */
//These are just wrappers around rspc functions
import {
  Client,
  createClient,
  FetchTransport,
  NoOpTransport,
  WebsocketTransport,
} from '@rspc/client';
import type { Procedures } from './bindings';

export function createFetchApiClient(url: string): Client<Procedures> {
  return createClient<Procedures>({
    transport: new FetchTransport(url),
  });
}

export function createWSApiClient(url: string): Client<Procedures> {
  return createClient<Procedures>({
    transport: new WebsocketTransport(url),
  });
}

export function createMockApiClient(): Client<Procedures> {
  return createClient<Procedures>({
    transport: new NoOpTransport(),
  });
}
