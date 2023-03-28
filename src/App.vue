<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Api from "./components/Api.vue";
import Audio from "./components/Audio.vue";
import Storage from "./components/Storage.vue";

import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const uuid = ref("");

async function machine() {
  uuid.value = await invoke("machine_id");
}

onMounted(() => {
  machine()
})
</script>

<template>
  <header data-tauri-drag-region class="faux-header"></header>

  <main class="h-screen flex flex-col items-center justify-center">
    <div class="max-w-3xl w-8/12 border borders divide-y border-dividers rounded-md shadow-md">
      <Audio class="p-4" />
      <Storage class="p-4" />
      <Api class="p-4" />
      <div class="p-4">
        <span class="font-bold">Machine ID: </span>
        <span>{{ uuid }}</span>
      </div>
    </div>
  </main>
</template>