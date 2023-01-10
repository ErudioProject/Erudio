import { createFetchApiClient } from "./lib/api";

export const FetchClient = createFetchApiClient(import.meta.env.FRONTEND_API_URL)
