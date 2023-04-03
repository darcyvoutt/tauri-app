<script>
import { initData, getData, saveData, resetData } from '../utils/data'
import { invoke } from '@tauri-apps/api/tauri'

export default {
  data() {
    return {
      data: null,
      customer_id: '',
    }
  },
  watch: {
    customer_id(customer_id) {
      this.save(customer_id)
    },
  },
  mounted() {
    this.init()
  },
  methods: {
    async init() {
      initData()

      const data = await getData()

      if (data === null || Object.keys(data).length === 0) return
      if (data.customer_id !== undefined) this.customer_id = data.customer_id
      else this.randomize()

      this.data = data
    },
    randomize() {
      const num = Math.floor(Math.random() * 899999 + 100000)
      this.customer_id = `${num}`
    },
    async save(customer_id) {
      let initData = await getData()

      if (initData === null) {
        await resetData()
        initData = await getData()
      }

      const updatedData = Object.assign(initData, { customer_id })

      this.$store.commit('saveSettings', updatedData)
      this.data = updatedData
      this.customer_id = customer_id

      await saveData(updatedData)

      const license = await invoke('hash_string', { customerId: customer_id })
      this.$store.commit('saveLicense', license)
    },
  },
}
</script>

<template>
  <div class="flex items-center gap-4">
    <form class="flex items-center space-x-4" @submit.prevent>
      <label for="customerId" class="font-bold">Settings Data:</label>
      <input
        id="customerId"
        class="input input-readonly"
        v-model="customer_id"
        readonly
      />
    </form>
    <button class="btn" @click.prevent="randomize()">Update</button>
    <pre class="pre flex-grow">{{ data }}</pre>
  </div>
</template>
