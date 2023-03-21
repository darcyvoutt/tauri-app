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
  <div class="pt-2">
    <form>
      <input type="text" v-model="input" class="py-2.5 px-4 border-2 border-neutral-300 rounded-md" />
    </form>

    <p>Storage: {{ storage }}</p>
  </div>
</template>