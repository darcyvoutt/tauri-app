<script setup>
import { ref, onMounted } from 'vue'
import { fetch } from '@tauri-apps/api/http';

const response = ref("No response");

async function apiCall() {
  return await fetch('https://api.publicapis.org/entries', {
    method: 'GET',
    timeout: 30,
  });
}

onMounted(() => { apiCall() })
</script>

<template>
  <div class="flex items-center space-x-6">
    <button class="btn my-4" @click="fetch()">Fetch Data</button>

    <div class="w-1/2 px-6 py-4 bg-neutral-200 text-neutral-900 rounded-md">
      {{ response }}
    </div>

  </div>
</template>