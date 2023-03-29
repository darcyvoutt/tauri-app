<script>
import { writeText } from '@tauri-apps/api/clipboard';
import { notify } from '../utils/notify'

export default {
  computed: {
    counterValue() {
      return this.$store.state.count
    },
  },
  methods: {
    async notification() {
      notify('This is a test notification')
      await writeText('Copied to clipboard with notification.')
    },
  }
}
</script>

<template>
  <div class="flex items-center gap-x-4">
    <button @click="notification" class="btn">
      Send Notification
    </button>
    <div class="italic">Only working when Alert type for MacOS</div>

    <div class="flex items-center space-x-4 pl-4 border-l border-neutral-600">
      <span>Count: </span>
      <pre class="pre">{{ counterValue }}</pre>
    </div>
  </div>
</template>