import { Client } from '@rspc/client';
import { createContext, JSX, ParentProps, useContext } from 'solid-js';
import { Procedures } from '../../../../bindings';
import {
    createFetchApiClient,
    createMockApiClient,
    createWSApiClient,
} from '../../lib/api';

interface ClientProviderProps {
    url: string;
    children?: JSX.Element;
}

const ClientContext = createContext<ClientContextType>();

interface ClientContextType {
    getFetchClient(): Client<Procedures>;
    getWsClient(): Client<Procedures>;
    getMockClient(): Client<Procedures>;
}

function ClientProvider(props: ParentProps<ClientProviderProps>) {
    let fetchClient: Client<Procedures> | null = null;
    let wsClient: Client<Procedures> | null = null;
    let mockClient: Client<Procedures> | null = null;
    const client: ClientContextType = {
        getFetchClient(): Client<Procedures> {
            if (fetchClient === null) fetchClient = createFetchApiClient(props.url);
            return fetchClient;
        },

        getWsClient(): Client<Procedures> {
            if (wsClient === null) wsClient = createWSApiClient(props.url);
            return wsClient;
        },

        getMockClient(): Client<Procedures> {
            if (mockClient === null) mockClient = createMockApiClient();
            return mockClient;
        },
    };

    return (
        <ClientContext.Provider value={client}>
            {props.children}
        </ClientContext.Provider>
    );
}

export function useClient() {
    return useContext(ClientContext)!;
}

export default ClientProvider;
