import { mount, StartClient } from 'solid-start/entry-client';
import { worker } from './test/mocks/browser';

if (import.meta.env.DEV) {
    worker.start()
}

mount(() => <StartClient />, document);
