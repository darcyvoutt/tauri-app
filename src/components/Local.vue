<script>
import { writeText } from '@tauri-apps/api/clipboard'

export default {
  data() {
    return {
      input: null,
      storage: 'Storage is empty',
    }
  },
  watch: {
    input(input) {
      window.localStorage.setItem('storage', input)
      this.storage = input
    },
    storage(storage) {
      if (storage.length === 0) this.storage = 'Storage is empty'
    },
  },
  mounted() {
    this.init()
  },
  methods: {
    async copy() {
      await writeText(this.storage)
      alert('Local storage copied to clipboard')
    },
    init() {
      const storage = localStorage.getItem('storage')
      if (storage === null) return
      this.storage = storage
      this.input = storage
    },
  },
}
</script>

<template>
  <div class="grid grid-cols-6 space-x-4">
    <form class="flex items-center space-x-4 col-span-3" @submit.prevent>
      <label for="Storage" class="font-bold flex-shrink-0">
        LocalStorage:
      </label>
      <input
        id="LocalStorage"
        type="text"
        v-model="input"
        class="input flex-grow min-w-0"
      />
    </form>

    <div class="col-span-3 flex items-center space-x-4">
      <pre class="pre flex-grow">{{ storage }}</pre>
      <button class="btn" @click="copy()">Copy</button>
    </div>
  </div>
</template>
