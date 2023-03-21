<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from 'vue'

import Audio from "./components/Audio.vue";
import Greet from "./components/Greet.vue";
import Welcome from "./components/Welcome.vue";

import { invoke } from "@tauri-apps/api/tauri";

async function getData() {
  return await invoke("fetch", { name: name.value });
}

onMounted(() => {
  this.getData()
})
</script>

<template>
  <div class="container">
    <Welcome />
    <Greet />
    <Audio />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
