<script>
import { initDataCheck, getData, saveData } from '../utils/data'

export default {
  data() {
    return {
      dataFile: 'No data found.',
      input: null,
    }
  },
  watch: {
    input() {
      this.saveData()
    },
  },
  mounted() {
    initDataCheck()
    this.updateData()
    this.updateInput()
  },
  methods: {
    async saveData() {
      const initData = await getData()
      const updatedData = Object.assign(initData, { input: this.input })
      await saveData(updatedData)
      this.updateData()
    },
    async updateData() {
      this.dataFile = await getData()
    },
    async updateInput() {
      const data = await getData()
      this.input = data.input
    },
  },
}
</script>

<template>
  <div class="flex items-center gap-4">
    <form
      class="flex items-center space-x-4"
      :class="loading ? 'animate-pulse' : ''"
      @submit.prevent
    >
      <label for="dataInput">Settings Data:</label>
      <input id="dataInput" class="input" v-model="input" />
    </form>
    <pre class="pre flex-grow">{{ dataFile }}</pre>
  </div>
</template>
