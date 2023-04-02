<script setup>
import { ref } from 'vue'
import axios from 'axios'
import axiosTauriAdapter from 'axios-tauri-adapter'

const client = axios.create({ adapter: axiosTauriAdapter })
const headers = { "Content-Type": "application/json" }
const getData = ref('Click Fetch Data')
const loading = ref(false)

async function getRequest() {
  getData.value = 'Loading...'
  loading.value = true
  const response = await client.get(
    'https://api.publicapis.org/entries',
    {
      headers
    }
  )
  getData.value = response.data.entries[0]
  loading.value = false
}
</script>

<template>
  <div class="flex items-center space-x-4">
    <button class="btn" @click="getRequest()">Fetch Data</button>

    <pre class="pre flex-grow text-sm" :class="loading ? 'animate-pulse' : ''">{{ getData }}</pre>
  </div>
</template>
