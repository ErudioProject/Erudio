import solid from 'solid-start/vite';
import { defineConfig } from 'vite';
import suidPlugin from '@suid/vite-plugin';

export default defineConfig({
  plugins: [
    suidPlugin.default(),
    solid({
      ssr: false,
    }),
  ],
  build: {
    target: 'esnext',
  },
  envPrefix: 'NX_',
});
