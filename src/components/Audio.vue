<script>
import file from "../assets/piano.mp3"
export default {
  data() {
    return {
      file: file,
      volume: 0.2,
    };
  },
  computed: {
    currentVolume() {
      return this.volume * 100
    },
  },
  methods: {
    play() {
      this.$refs.audio.volume = this.volume
      this.$refs.audio.play()
    },
    increment() {
      this.$store.commit('increment')
    },
  }
};
</script>

<template>
  <div class="flex items-center justify-between">
    <div class="flex items-center space-x-4">
      <button class="btn" @click="play()">
        Play Audio
      </button>

      <input type="range" min="0" max="1" step="0.1" v-model="volume">
      <span>{{ currentVolume }}</span>

      <audio ref="audio" :src="file"></audio>
    </div>

    <button class="btn" @click.prevent="increment()">
      Increment Counter
    </button>
  </div>
</template>
