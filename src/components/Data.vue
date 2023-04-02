<script>
import { initData } from '../utils/data'

export default {
  data() {
    return {
      input: null,
      dataFile: 'No data found.',
    }
  },
  mounted() {
    initData()
  },
}
</script>

<template>
  <div class="flex items-center gap-4">
    <form @submit.prevent>
      <label for="dataInput"></label>
      <input id="dataInput" class="input" v-model="input" />
    </form>
    <pre class="pre">{{ dataFile }}</pre>
  </div>
</template>
