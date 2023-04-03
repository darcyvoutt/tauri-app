<script setup>
import Api from './components/Api.vue'
import Audio from './components/Audio.vue'
import Data from './components/Data.vue'
import Increment from './components/Increment.vue'
import Local from './components/Local.vue'
import Machine from './components/Machine.vue'
import Notification from './components/Notification.vue'
import Shell from './components/Shell.vue'
</script>

<template>
  <header data-tauri-drag-region class="faux-header"></header>

  <main class="h-screen flex flex-col items-center justify-center">
    <div
      class="max-w-4xl w-10/12 border borders divide-y border-dividers rounded-md shadow-md"
    >
      <div class="p-4 flex items-center justify-between">
        <Audio />
        <Shell />
        <Increment />
      </div>
      <Notification class="p-4" />
      <Api class="p-4" />
      <Local class="p-4" />
      <Data class="p-4" />
      <Machine class="p-4" />
    </div>
  </main>
</template>
