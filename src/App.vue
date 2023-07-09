<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

interface MediaDevice {
  id: number
  name: string
  type: 'input' | 'output'
  uid: string
}

const preferredAudioDeviceName = 'Beats Fit Pro'
const currentOuputDeviceName = ref('MacBook Pro Speakers')
const initialDelayDone = ref(false)
const webMediaDevics = ref<MediaDeviceInfo[]>([])
const outputDevices = ref<MediaDevice[]>([])
const inputDevices = ref<MediaDevice[]>([])

async function getDefaultOutput() {
  console.log('>> getDefaultOutput')
  invoke<{ name: string }>('get_default_output_name')
    .then((result) => {
      console.log('getDefaultOutput result', result)
      currentOuputDeviceName.value = result.name
    })
    .catch((error) => {
      console.error(error)
    })
}

async function updateDevices() {
  console.log('>> updateDevices')
  navigator.mediaDevices.enumerateDevices().then((devices) => {
    const newDevices: MediaDeviceInfo[] = []
    devices.forEach((device) => {
      if (device.kind.startsWith('audio')) {
        newDevices.push(device)
      }
    })
    webMediaDevics.value = newDevices
    console.log('devices', devices)
    getDefaultOutput()
  })

  invoke<MediaDevice[]>('list_output_devices')
    .then((result) => {
      console.log('output result', result)
      outputDevices.value = result
    })
    .catch((error) => {
      console.error(error)
    })
}

async function setAudioOutput(name: string) {
  console.log('setAudioOutput: ', name)

  if (name !== currentOuputDeviceName.value) {
    invoke('set_system_audio_output', { name: name })
      .then(() => {
        // currentlySelectedOutput.value = name
        getDefaultOutput()
        console.log('set_system_audio_output done')
      })
      .catch((error) => {
        console.error(error)
        console.log('set_system_audio_output FAILED')
      })
  } else {
    console.log(
      'set_system_audio_output SKIPPED. currently selected: ',
      currentOuputDeviceName.value
    )
  }
}

onMounted(async () => {
  await navigator.mediaDevices.getUserMedia({ video: false, audio: true })

  updateDevices()
  // await setAudioOutput(defaultAudioDeviceName)

  navigator.mediaDevices.ondevicechange = async () => {
    console.log('devices changed')
    await setAudioOutput(preferredAudioDeviceName)
    updateDevices()
  }

  // invoke<MediaDevice[]>('list_input_devices')
  //   .then((result) => {
  //     console.log('input result', result)
  //     inputDevices.value = result
  //   })
  //   .catch((error) => {
  //     console.error(error)
  //   })
})

setTimeout(() => {
  initialDelayDone.value = true
}, 2000)
</script>

<template>
  <div class="flex flex-col p-2">
    <div>
      <h3>deviceList devices</h3>

      <ul class="py-4 list-disc list-inside">
        <li v-for="device in outputDevices" :key="device.id">
          {{ device.name }}
          <span v-if="currentOuputDeviceName === device.name"> (selected) </span>
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
