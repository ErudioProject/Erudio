import ClientProvider, { useClient } from './ClientProvider';
import { render } from 'solid-testing-library';
import { Component } from 'solid-js';
import * as Data from '../../lib/api';
import { describe, it, expect, vi } from 'vitest';

const createFetch = vi
    .spyOn(Data, 'createFetchApiClient')
    .mockImplementation((_: string) => Data.createMockApiClient());
const createWS = vi
    .spyOn(Data, 'createWSApiClient')
    .mockImplementation((_: string) => Data.createMockApiClient());
const createMock = vi.spyOn(Data, 'createMockApiClient');

const Child: Component = () => {
    const client = useClient()!;
    const fetch = client.getFetchClient();
    const ws = client.getWsClient();
    const mock = client.getMockClient();
    const fetch2 = client.getFetchClient();
    const ws2 = client.getWsClient();
    const mock2 = client.getMockClient();

    return (
        <>
            Client: {client.toString()}
            Fetch: {fetch.toString()}
            WS: {ws.toString()}
            Mock: {mock.toString()}
            Fetch2: {fetch2.toString()}
            WS2: {ws2.toString()}
            Mock2: {mock2.toString()}
        </>
    );
};

const App: Component = () => {
    return (
        <ClientProvider url="">
            <Child />
        </ClientProvider>
    );
};
describe('ClientProvider', () => {
    render(() => <App />);
    it('calls create clients only once', () => {
        expect(createFetch).toHaveBeenCalledTimes(1);
        expect(createWS).toHaveBeenCalledTimes(1);
        expect(createMock).toHaveBeenCalledTimes(1 + 2 /*From the other mocks*/);
    });
});
