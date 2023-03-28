<script setup>
import { ref } from 'vue'
import axios from 'axios'
import axiosTauriAdapter from 'axios-tauri-adapter'

const client = axios.create({ adapter: axiosTauriAdapter })
const headers = { "Content-Type": "application/json" }
const data = ref('No data')

async function apiCall() {
  const response = await client.post('https://m32m1.mocklab.io/json',
    {
      id: 12345,
      name: "James"
    },
    headers
  )
  data.value = response.data
}
</script>

<template>
  <div class="grid grid-cols-8 items-center space-x-4">
    <button class="btn col-span-2" @click="apiCall()">Fetch Data</button>

    <pre class="col-span-6 border borders p-3.5 px-8 rounded-md">{{
      data
    }}</pre>
  </div>
</template>
