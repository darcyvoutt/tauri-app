<script setup>
import file from "../assets/piano.mp3"
const audioFile = file
</script>

<template>
  <audio ref="audio" :src="audioFile" controls></audio>

  <button @click="$refs.audio.play()">Play Audio</button>
</template>
