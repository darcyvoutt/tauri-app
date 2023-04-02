<script>
import { appDataDir } from '@tauri-apps/api/path'

export default {
  data() {
    return {
      input: null,
    };
  },
  mounted() {
    this.appDirs()
  },
  methods: {
    async appDirs() {
      console.log('APP DATA:', await appDataDir())
    }
  },
}
</script>

<template>
  <div>

  </div>
</template>