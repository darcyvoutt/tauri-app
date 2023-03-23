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
  <div>
    <audio ref="audio" :src="file"></audio>

    <button class="btn" @click="play()">
      Play Audio
    </button>
  </div>
</template>
