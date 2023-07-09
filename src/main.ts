import devtools from '@vue/devtools'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { useStore } from './store'

import './assets/main.postcss'

if (process.env.NODE_ENV === 'development') {
  devtools.connect('http://localhost', 8098)
}

const app = createApp(App)
const pinia = createPinia()
app.use(pinia)

const store = useStore(pinia)
store.init().then(() => {
  console.log('main.ts: store initialized')
})

app.mount('#app')
