import { createApp } from "vue"
import { store } from "./stores/index"
import App from "./App.vue"

import "../dist/main.css"

const app = createApp(App)

app.use(store)
app.mount("#app")