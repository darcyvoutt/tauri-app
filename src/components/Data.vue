<script>
import { initDataCheck, getData, saveData } from '../utils/data'

export default {
  data() {
    return {
      input: null,
      dataFile: 'No data found.',
    }
  },
  watch: {
    input() {
      this.updateData()
    },
  },
  mounted() {
    initDataCheck()
  },
  methods: {
    async updateData() {
      const data = await getData()
      const updatedData = Object.assign(data, { input: this.input })
      await saveData(updatedData)
    },
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
