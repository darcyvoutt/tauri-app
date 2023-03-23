<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Audio from "./components/Audio.vue";
import Storage from "./components/Storage.vue";

import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const response = ref("");

async function fetch() {
  response.value = await invoke("fetch_data", { test: `Date & Time: ${new Date().toUTCString()}` });
}

onMounted(() => {
  fetch()
})
</script>

<template>
  <div>
    <header data-tauri-drag-region class="faux-header"></header>
    <div class="h-screen flex flex-col items-center justify-center">
      <Audio />
      <Storage />

      <div class="pt-4 w-full flex flex-col items-center">
        <button class="btn my-4" @click="fetch()">Fetch Data</button>
        <div class="w-1/2 p-8 bg-neutral-200 text-neutral-900 rounded-md">
          {{ response }}
        </div>
      </div>
    </div>
  </div>
</template>