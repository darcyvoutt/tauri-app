<script>
import { writeText } from '@tauri-apps/api/clipboard';
import { notify } from '../utils/notify'

export default {
  methods: {
    async notification() {
      notify('This is a test notification')
      await writeText('Copied to clipboard with notification.');
    }
  }
}
</script>

<template>
  <div class="flex items-center gap-x-4">
    <button @click="notification" class="btn">
      Send Notification
    </button>
    <p class="italic">Only working when Alert type for MacOS</p>
  </div>
</template>