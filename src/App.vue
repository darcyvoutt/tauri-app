<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Audio from "./components/Audio.vue";
import Storage from "./components/Storage.vue";
</script>

<template>
  <div>
    <header data-tauri-drag-region class="faux-header"></header>
    <div class="h-screen flex flex-col items-center justify-center">
      <Audio />
      <Storage />
    </div>
  </div>
</template>