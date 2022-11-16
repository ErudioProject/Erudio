import ClientProvider, { useClient } from './ClientProvider'
import { render } from 'solid-testing-library'
import { Component } from 'solid-js';
import * as Data from '../../lib/frontend-data-access-api';

const createFetch = jest.spyOn(Data, 'createFetchApiClient');
const createWS = jest.spyOn(Data, 'createWSApiClient');
const createMock = jest.spyOn(Data, 'createMockApiClient');

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
            Client: {client}
            Fetch: {fetch}
            WS: {ws}
            Mock: {mock}
            Fetch2: {fetch2}
            WS2: {ws2}
            Mock2: {mock2}
        </>
    );
}

const App: Component = () => {
    return (
        <ClientProvider url=''>
            <Child />
        </ClientProvider>
    );
}
describe("ClientProvider", () => {
    render(<App />);
    it("calls create functions only once", () => {
        expect(createFetch).toHaveBeenCalledTimes(1);
        expect(createWS).toHaveBeenCalledTimes(1);
        expect(createMock).toHaveBeenCalledTimes(1);
    });
});
