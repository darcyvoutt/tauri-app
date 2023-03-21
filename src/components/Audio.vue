<script>
import file from "../assets/piano.mp3"
export default {
  data() {
    return {
      file: file,
    };
  },
  methods: {
    play() {
      this.$refs.audio.volume = 0.5;
      this.$refs.audio.play();
    },
  }
};
</script>

<template>
  <audio ref="audio" :src="file"></audio>

  <button
    class="bg-teal-700 text-white font-bold py-3 px-6 rounded-lg"
    @click="play()"
  >
    Play Audio
  </button>
</template>
