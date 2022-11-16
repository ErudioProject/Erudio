import { createFetchApiClient } from "./frontend-data-access-api";
import 'isomorphic-fetch';

global.fetch = fetch
const client = createFetchApiClient(process.env.NX_API_URL!)

describe("createFetchApiClient", () => {
    it("returns version correctly", async () => {
        const res = await client.query(["public.version"]);
        expect(res).toMatch(/\d+\.\d+\.\d+/);
    })
})
