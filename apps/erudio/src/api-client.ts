import { createFetchApiClient } from "@erudio/frontend/data-access/api";
export const fetchClient = createFetchApiClient(import.meta.env.API_URL)
