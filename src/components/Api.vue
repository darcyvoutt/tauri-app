<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const data = ref('No response')

async function apiCall() {
  data.value = await invoke("post_request", {
    data: {
      username: "Darcy",
      age: 37,
      is_active: true
    }
  });
}

onMounted(() => {
  apiCall()
})
</script>

<template>
  <div class="grid grid-cols-8 items-center space-x-4">
    <button class="btn col-span-2" @click="apiCall()">Fetch Data</button>

    <pre class="col-span-6 border borders p-3.5 px-8 rounded-md">{{
      data
    }}</pre>
  </div>
</template>
