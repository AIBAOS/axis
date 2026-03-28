import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

<<<<<<< HEAD
// https://vitejs.dev/config/
=======
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
<<<<<<< HEAD
        target: process.env.VITE_API_BASE_URL || 'http://localhost:8080',
        changeOrigin: true
      }
    }
=======
        target: process.env.VITE_API_URL || 'http://localhost:8080',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    assetsDir: 'static'
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)
  }
})
