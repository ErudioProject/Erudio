/// <reference types="vitest" />
import solid from 'solid-start/vite';
import { defineConfig } from 'vite';
import suidPlugin from '@suid/vite-plugin';
import { defineDefaultConfig } from "../../../vite.config.base";

export default defineConfig({
  ...defineDefaultConfig,
  plugins: [
    /* @ts-ignore */
    suidPlugin.default(),
    solid({
      ssr: false,
    }),
  ],
});
