import { createClient, FetchTransport } from "@rspc/client";
import { createSolidQueryHooks } from "@rspc/solid";
import { QueryClient } from "@tanstack/solid-query";
import { Procedures } from "../../bindings";
import clientEnv from "./lib/env";

// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
export const client = createClient<Procedures>({
    // Refer to the integration your using for the correct transport.
    transport: new FetchTransport(clientEnv.VITE_API_URL),
});

export const queryClient = new QueryClient();
const rspc = createSolidQueryHooks<Procedures>();

export default rspc;
