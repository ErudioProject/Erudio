import { createFetchApiClient } from "./frontend-data-access-api";
import 'isomorphic-fetch';

// TODO patryk you are fixing it move it to jest.init.ts or somewhere
import { config } from 'dotenv';
if (!process.env['NX_INVOKED_BY_RUNNER']) {
    config({ path: __dirname + '/.env' });
}

global.fetch = fetch
const client = createFetchApiClient(process.env.VITE_API_URL!)

describe("createFetchApiClient", () => {
    it("returns version correctly", async () => {
        const res = await client.query(["public.version"]);
        expect(res).toMatch(/\d+\.\d+\.\d+/);
    })
})
