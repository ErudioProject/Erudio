import { Client } from '@rspc/client';
import { createContext, JSX, useContext } from 'solid-js';
import { Procedures } from '../../lib/bindings';
import {
  createFetchApiClient,
  createMockApiClient,
  createWSApiClient,
} from '../../lib/frontend-data-access-api';

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

function ClientProvider(props: ClientProviderProps) {
  let fetchClient: Client<Procedures> | null = null;
  let wsClient: Client<Procedures> | null = null;
  let mockClient: Client<Procedures> | null = null;
  const client: ClientContextType = {
    getFetchClient() {
      if (fetchClient === null) fetchClient = createFetchApiClient(props.url);
      return fetchClient;
    },

    getWsClient() {
      if (wsClient === null) wsClient = createWSApiClient(props.url);
      return wsClient;
    },

    getMockClient() {
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
  return useContext<ClientContextType | undefined>(ClientContext);
}

export default ClientProvider;
