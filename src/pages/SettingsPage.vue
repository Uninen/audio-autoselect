<script setup lang="ts">
const store = useStore()
const currentPage = computed(() => store.currentPage)
</script>

<template>
  <div v-if="currentPage === 'settings'" class="flex flex-col h-full">
    <div class="flex flex-col flex-grow">
      <h2>Output devices</h2>

      <ul class="mt-2 border rounded-md border-white/10">
        <li
          v-for="(device, idx) in store.outputDevices"
          :key="device"
          class="px-4 py-2"
          :class="{
            'bg-white/20': store.currentOuputDeviceName === device,
            'rounded-t-md': idx === 0,
          }"
        >
          {{ device }}
        </li>
      </ul>
    </div>
    <div class="flex justify-end flex-grow-0">
      <button class="btn" @click="store.quitApp"> Quit </button>
    </div>
  </div>
</template>
