import { createStore } from 'vuex'

// Create a new store instance.
export const store = createStore({
  state() {
    return {
      count: 0,
      license: '',
      settings: {},
    }
  },
  mutations: {
    increment(state) {
      state.count++
    },
    saveLicense(state, str) {
      state.license = str
    },
    saveSettings(state, obj) {
      if (typeof obj !== 'object' || obj === null) return
      state.settings = obj
    },
  },
})
