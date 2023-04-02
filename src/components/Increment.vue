<script setup>
/**
 * Example of Vuex method being fired from one component
 * and the data updating in a completely different component.
 */
</script>

<template>
  <button class="btn" @click.prevent="$store.commit('increment')">
    Increment Counter
  </button>
</template>
