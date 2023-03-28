<script setup>
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
  <div class="flex items-center space-x-4">
    <span class="font-bold">Machine ID: </span>
    <pre class="pre flex-grow">{{ uuid }}</pre>
  </div>
</template>
