// vite.config.ts
import solid from 'file:///home/patryk/Projects/personal/Erudio/node_modules/.pnpm/solid-start@0.2.3_vqtaenuqfpqmxqu2kobz7unc2m/node_modules/solid-start/vite/plugin.js';
import { defineConfig } from 'file:///home/patryk/Projects/personal/Erudio/node_modules/.pnpm/vite@3.2.2/node_modules/vite/dist/node/index.js';
import suidPlugin from 'file:///home/patryk/Projects/personal/Erudio/node_modules/.pnpm/@suid+vite-plugin@0.0.3_vite@3.2.2/node_modules/@suid/vite-plugin/index.cjs';
var vite_config_default = defineConfig({
  plugins: [
    suidPlugin.default(),
    solid({
      ssr: false,
    }),
  ],
  build: {
    target: 'esnext',
  },
});
export { vite_config_default as default };
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9wYXRyeWsvUHJvamVjdHMvcGVyc29uYWwvRXJ1ZGlvL2FwcHMvZXJ1ZGlvXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvaG9tZS9wYXRyeWsvUHJvamVjdHMvcGVyc29uYWwvRXJ1ZGlvL2FwcHMvZXJ1ZGlvL3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9ob21lL3BhdHJ5ay9Qcm9qZWN0cy9wZXJzb25hbC9FcnVkaW8vYXBwcy9lcnVkaW8vdml0ZS5jb25maWcudHNcIjtpbXBvcnQgc29saWQgZnJvbSAnc29saWQtc3RhcnQvdml0ZSc7XG5pbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlJztcbmltcG9ydCBzdWlkUGx1Z2luIGZyb20gXCJAc3VpZC92aXRlLXBsdWdpblwiO1xuXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xuICBwbHVnaW5zOiBbc3VpZFBsdWdpbi5kZWZhdWx0KCksIHNvbGlkKHtcbiAgICBzc3I6IGZhbHNlLFxuICB9KV0sXG4gIGJ1aWxkOiB7XG4gICAgdGFyZ2V0OiAnZXNuZXh0JyxcbiAgfVxufSk7XG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQXFVLE9BQU8sV0FBVztBQUN2VixTQUFTLG9CQUFvQjtBQUM3QixPQUFPLGdCQUFnQjtBQUV2QixJQUFPLHNCQUFRLGFBQWE7QUFBQSxFQUMxQixTQUFTLENBQUMsV0FBVyxRQUFRLEdBQUcsTUFBTTtBQUFBLElBQ3BDLEtBQUs7QUFBQSxFQUNQLENBQUMsQ0FBQztBQUFBLEVBQ0YsT0FBTztBQUFBLElBQ0wsUUFBUTtBQUFBLEVBQ1Y7QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
