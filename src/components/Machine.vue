<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const uuid = ref('')
const license = ref('')

async function machine() {
  uuid.value = await invoke('machine_id')
}

async function license_key() {
  license.value = await invoke('hash_string')
}

onMounted(() => {
  machine()
  license_key()
})
</script>

<template>
  <div class="space-y-4">
    <div class="grid grid-cols-6 items-center space-x-4">
      <span class="col-span-1 font-bold">Machine ID: </span>
      <pre class="col-span-5 pre">{{ uuid }}</pre>
    </div>

    <div class="grid grid-cols-6 items-center space-x-4">
      <span class="col-span-1 font-bold">License: </span>
      <pre class="col-span-5 pre">{{ license }}</pre>
    </div>
  </div>
</template>
