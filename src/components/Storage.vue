<script>
export default {
  data() {
    return {
      input: null,
      storage: null,
    };
  },
  watch: {
    input(input) {
      localStorage.setItem("storage", input)
      this.storage = input
    },
  },
  mounted() {
    const storage = localStorage.getItem("storage")
    this.storage = storage
    this.input = storage
  },
}
</script>

<template>
  <div class="grid grid-cols-6 space-x-4">
    <form
      class="flex items-center space-x-4 col-span-3"
      @submit.prevent
    >
      <label for="Storage" class="font-bold flex-shrink-0">
        LocalStorage:
      </label>
      <input
        id="Storage"
        type="text"
        v-model="input"
        class="input flex-shrink min-w-0"
      />
    </form>

    <pre class="pre col-span-3">{{ storage }}</pre>
  </div>
</template>