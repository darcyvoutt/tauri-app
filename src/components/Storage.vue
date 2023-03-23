<script>
export default {
  data() {
    return {
      input: null,
      storage: null,
    }
  },
  watch: {
    input(input) {
      localStorage.setItem("test", input)
    }
  },
  mounted() {
    this.storage = localStorage.getItem("test")
  },
}
</script>

<template>
  <div class="flex items-center space-x-6">
    <form>
      <label for="Storage" class="mr-2">
        Storage:
      </label>
      <input id="Storage" type="text" v-model="input" class="py-2.5 px-4 border-2 border-neutral-300 rounded-md" />
    </form>

    <pre class="border borders p-3.5 px-8 rounded-md">{{ storage }}</pre>
  </div>
</template>