/// <reference types="vitest" />
import solid from 'solid-start/vite';
import { defineConfig, loadEnv } from 'vite';
import { defineDefaultConfig } from "../../../vite.config.base";

import { VitePWA } from "vite-plugin-pwa";

import path from "path";
import crypto from "crypto";
import fs from "fs";

const envDir = '../../../';

export default defineConfig(({ mode }) => {
  // Load env file based on `mode` in the current working directory.
  // Set the third parameter to '' to load all env regardless of the `VITE_` prefix.
  const env = loadEnv(mode, envDir, '')

  const indexHtmlRevision = () => {
    // Environment variable set only when building the client.
    // See <https://github.com/solidjs/solid-start/blob/df5d22be3db0f76e4ab5d815c1892855ec43b1f2/packages/start/bin.cjs#L398>.
    if (!env.START_SPA_CLIENT) return "";

    const index_path = path.resolve(__dirname, ".solid/index.html");
    const file_buffer = fs.readFileSync(index_path);
    const hash = crypto.createHash("md5");
    hash.update(file_buffer);
    return hash.digest("hex");
  };

  return {
    ...defineDefaultConfig,
    envDir,
    plugins: [
      VitePWA({
        workbox: {
          additionalManifestEntries: [
            {
              url: "index.html",
              revision: indexHtmlRevision()
            }
          ]
        }
      }),
      solid({
        ssr: false,
      }),
    ],
  }
})
