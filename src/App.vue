<script setup lang="ts">
// import { invoke } from '@tauri-apps/api/tauri'

interface MediaDevice {
  id: number
  name: string
  type: 'input' | 'output'
  uid: string
}

const outputDevices = ref<MediaDevice[]>([])
const inputDevices = ref<MediaDevice[]>([])

const deviceList = ref<MediaDeviceInfo[]>([])

function updateDevices() {
  navigator.mediaDevices.enumerateDevices().then((devices) => {
    const newDevices: MediaDeviceInfo[] = []
    devices.forEach((device) => {
      if (device.kind === 'audioinput') {
        newDevices.push(device)
      }
    })
    deviceList.value = newDevices
    console.log('devices', devices)
  })
}

onMounted(async () => {
  await navigator.mediaDevices.getUserMedia({ video: false, audio: true })

  updateDevices()

  navigator.mediaDevices.ondevicechange = () => {
    console.log('devices changed')
    updateDevices()
  }
  // invoke<MediaDevice[]>('list_output_devices')
  //   .then((result) => {
  //     console.log('output result', result)
  //     outputDevices.value = result
  //   })
  //   .catch((error) => {
  //     console.error(error)
  //   })
  // invoke<MediaDevice[]>('list_input_devices')
  //   .then((result) => {
  //     console.log('input result', result)
  //     inputDevices.value = result
  //   })
  //   .catch((error) => {
  //     console.error(error)
  //   })
})
</script>

<template>
  <div class="flex flex-col p-2">
    <div>
      <h3>deviceList devices</h3>

      <ul class="py-4 list-disc list-inside">
        <li v-for="device in deviceList" :key="device.deviceId">
          {{ device.label }}
        </li>
      </ul>
    </div>
    <div v-if="false">
      <h3>Output devices</h3>

      <ul>
        <li v-for="device in outputDevices" :key="device.id">
          {{ device.name }}
        </li>
      </ul>
    </div>
    <div v-if="false">
      <h3>Input devices</h3>
      <ul>
        <li v-for="device in inputDevices" :key="device.id">
          {{ device.name }}
        </li>
      </ul>
    </div>
  </div>
</template>
