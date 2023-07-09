<script setup lang="ts">
import { useStore } from './store'

const store = useStore()
const isInited = computed(() => store.isInited)
</script>

<template>
  <div class="flex flex-col h-full p-2">
    <div v-if="isInited">
      <h3>Output devices</h3>

      <ul class="py-4 list-disc list-inside">
        <li v-for="device in store.outputDevices" :key="device">
          {{ device }}
          <span v-if="store.currentOuputDeviceName === device"> (selected) </span>
        </li>
      </ul>

      <button @click="store.quitApp"> Quit </button>
    </div>
    <div v-else class="flex items-center justify-center flex-1 w-full h-full"> Good Hello! </div>
  </div>
</template>
