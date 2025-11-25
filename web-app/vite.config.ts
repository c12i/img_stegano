import react from '@vitejs/plugin-react'
import {defineConfig} from 'vite'
import topLevelAwait from 'vite-plugin-top-level-await'
import wasm from 'vite-plugin-wasm'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  // Set base path for GitHub Pages deployment
  // Use repository name if deploying to https://username.github.io/repo-name/
  // Leave as '/' if deploying to custom domain or https://username.github.io/
  base: process.env.GITHUB_PAGES ? '/img_stegano/' : '/',
  build: {
    // Ensure WASM files are properly handled
    target: 'esnext',
    // Increase chunk size warning limit for WASM
    chunkSizeWarningLimit: 1000,
  },
})
