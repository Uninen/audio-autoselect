<script setup lang="ts">
const store = useStore()
const currentPage = computed(() => store.currentPage)
</script>

<template>
  <div
    v-if="currentPage === 'splash'"
    class="flex flex-col items-center justify-center flex-1 w-full h-full"
  >
    <div class="flex flex-col items-center justify-center flex-grow text-center text-white">
      Audio Autoselect Initializing...
    </div>
    <div class="flex justify-end flex-grow-0 py-2 text-xs opacity-30">
      Photo by Juan Di Nella on Unsplash
    </div>
  </div>
</template>
