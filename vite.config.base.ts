/// <reference types="vitest" />

import { UserConfigExport } from "vite";

export const defineDefaultConfig: UserConfigExport = {
    build: {
        target: 'esnext',
    },
    test: {
        globals: true,
        environment: 'jsdom',
        transformMode: {
            web: [/\.tsx?$/],
        },
        //TODO: find a way not to copy this
        setupFiles: './src/test/setupTest.ts',
        // solid needs to be inline to work around
        // a resolution issue in vitest
        // And solid-testing-library needs to be here so that the 'hydrate'
        // method will be provided
        deps: {
            inline: [/solid-js/, /solid-testing-library/],
        },
    },
    resolve: {
        conditions: ['development', 'browser'],
    },
    envPrefix: 'FRONTEND_',
};
