<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  data() {
    return {
      uuid: null,
    }
  },
  computed: {
    license() {
      return this.$store.state.license
    },
  },
  mounted() {
    this.machine()
  },
  methods: {
    async machine() {
      this.uuid = await invoke('machine_id')
    },
  },
}
</script>

<template>
  <div class="space-y-4">
    <div class="grid grid-cols-6 items-center space-x-4">
      <span class="col-span-1 font-bold">License: </span>
      <pre class="col-span-5 pre">{{ license }}</pre>
    </div>

    <div class="grid grid-cols-6 items-center space-x-4">
      <span class="col-span-1 font-bold">Machine ID: </span>
      <pre class="col-span-5 pre">{{ uuid }}</pre>
    </div>
  </div>
</template>
