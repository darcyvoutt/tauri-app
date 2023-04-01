<script setup>
import Api from "./components/Api.vue";
import Audio from "./components/Audio.vue";
import Local from "./components/Local.vue";
import Machine from "./components/Machine.vue";
import Notification from "./components/Notification.vue";
</script>

<template>
  <header data-tauri-drag-region class="faux-header"></header>

  <main class="h-screen flex flex-col items-center justify-center">
    <div class="max-w-4xl w-10/12 border borders divide-y border-dividers rounded-md shadow-md">
      <Audio class="p-4" />
      <Local class="p-4" />
      <Api class="p-4" />
      <Machine class="p-4" />
      <Notification class="p-4" />
    </div>
  </main>

</template>