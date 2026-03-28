import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
<<<<<<< HEAD
import './styles/index.css'
=======
import './styles/main.css'
>>>>>>> a37251b (feat(webui): Phase 301 创建 WebUI 基础框架)

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

app.mount('#app')
