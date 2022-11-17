import { createFetchApiClient } from './frontend-data-access-api';
import 'isomorphic-fetch';

global.fetch = fetch;
const client = createFetchApiClient(process.env.NX_API_URL!);

describe('createFetchApiClient', () => {
  it('returns version correctly', async () => {
    const res = await client.query(['public.version']);
    expect(res).toMatch(
      /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$/
    );
  });
});
