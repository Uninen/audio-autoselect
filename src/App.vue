<script setup lang="ts">
import { useStore } from './store'

const store = useStore()

console.log('Hello App.vue')
</script>

<template>
  <div class="flex flex-col p-2">
    <div>
      <h3>Output devices</h3>

      <ul class="py-4 list-disc list-inside">
        <li v-for="device in store.outputDevices" :key="device">
          {{ device }}
          <span v-if="store.currentOuputDeviceName === device"> (selected) </span>
        </li>
      </ul>
    </div>
  </div>
</template>
