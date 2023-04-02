<script setup>
import { open } from '@tauri-apps/api/shell';

async function openUrl(openWith = null) {
  if (openWith !== null) {
    await open('https://tauri.app', openWith)
    return
  }

  await open('https://tauri.app');
}
</script>

<template>
  <button class="btn" @click.prevent="openUrl()">
    Open URL
  </button>
  <button class="btn" @click.prevent="openUrl('firefox')">
    Open URL in FF
  </button>
</template>