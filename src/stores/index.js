import { createStore } from 'vuex'

// Create a new store instance.
export const store = createStore({
  state() {
    return {
      count: 0,
      settings: {},
    }
  },
  mutations: {
    increment(state) {
      state.count++
    },
    saveSettings(state, obj) {
      if (typeof obj !== 'object' || obj === null) return
      state.settings = obj
    },
  },
})
