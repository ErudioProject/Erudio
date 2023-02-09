import { mount, StartClient } from 'solid-start/entry-client';

if (import.meta.env.DEV) {
    const msw = await import("./test/mocks/browser")
    msw.worker.start()
}

mount(() => <StartClient />, document);
