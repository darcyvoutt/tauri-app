<script setup>
import { ref, onMounted } from 'vue'
import { fetch } from '@tauri-apps/api/http'

const apiUrl = 'https://api.publicapis.org/entries'

const data = ref('No response')

async function apiCall() {
  const response = await fetch(apiUrl, {
    method: 'GET',
    timeout: 30,
  });
  data = response
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
