import { createFetchApiClient } from "./frontend-data-access-api";

const client = createFetchApiClient(process.env.VITE_API_URL!)

describe("createFetchApiClient", () => {
    it("returns version correctly", async () => {
        const res = await client.query(["public.version"]);
        expect(res).not.toBeNull();
    })
})
