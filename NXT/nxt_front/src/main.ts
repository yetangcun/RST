import './assets/main.css'
import 'element-plus/dist/index.css'
import './assets/iconfont/iconfont.css'
import './assets/iconfont/iconfont.js'

import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

const app = createApp(App)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.use(ElementPlus, { size: 'large', zIndex: 3000 })

app.use(createPinia())
app.use(router)

app.mount('#app')
