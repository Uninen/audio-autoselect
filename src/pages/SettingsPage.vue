<script setup lang="ts">
const store = useStore()
const currentPage = computed(() => store.currentPage)
const setupDone = computed(() => store.setupDone)
</script>

<template>
  <div v-if="currentPage === 'settings'" class="flex flex-col h-full">
    <div class="flex flex-col flex-grow">
      <h2>Settings</h2>

      <form>
        <div class="flex flex-col mt-2">
          <label for="default-device">Default device name</label>
          <input v-model="store.systemDefaultAudioDeviceName" type="text" name="default-device" />
        </div>
        <div class="flex flex-col mt-2">
          <label for="default-device">Preferred device name</label>
          <input v-model="store.preferredAudioDeviceName" type="text" name="default-device" />
        </div>
      </form>
    </div>
    <div class="flex justify-end flex-grow-0 space-x-2">
      <button v-if="setupDone" class="btn" @click="store.setPage('main')"> Cancel </button>
      <button class="btn primary" @click="store.saveSettings"> Save </button>
    </div>
  </div>
</template>
