<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

const systemDefaultAudioDeviceName = 'MacBook Pro Speakers'
const preferredAudioDeviceName = 'Beats Fit Pro'
const currentOuputDeviceName = ref('MacBook Pro Speakers')
const initialDelayDone = ref(false)
const outputDevices = ref<string[]>([])
const oldDeviceCount = ref(0)
const outputDeviceCount = computed(() => outputDevices.value.length)
const furHatDebounce = ref(false)

async function getDefaultOutput() {
  console.log('>> getDefaultOutput')
  invoke<{ name: string }>('get_default_output_name')
    .then((result) => {
      console.warn('currentOuputDeviceName set:', result)
      currentOuputDeviceName.value = result.name
    })
    .catch((error) => {
      console.error(error)
    })
}

async function updateDevices() {
  console.log('>> updateDevices')
  oldDeviceCount.value = outputDeviceCount.value
  console.log('oldDeviceCount: ', oldDeviceCount.value)

  invoke<string[]>('list_output_devices')
    .then((result) => {
      outputDevices.value = result
      console.log('outputDevices: ', result)
    })
    .catch((error) => {
      console.error(error)
    })
}

async function setAudioOutput(name: string) {
  console.log('>> setAudioOutput', name)

  if (name !== currentOuputDeviceName.value) {
    console.log('invoking set_system_audio_output:', name)
    invoke('set_system_audio_output', { name: name })
      .then(() => {
        // currentlySelectedOutput.value = name
        getDefaultOutput()
        console.log('set_system_audio_output done')
        updateDevices()

        setTimeout(() => {
          getDefaultOutput()
        }, 2000)
      })
      .catch((error) => {
        console.error(error)
        console.warn('set_system_audio_output FAILED')
      })
  } else {
    console.log(
      'set_system_audio_output SKIPPED. currently selected: ',
      currentOuputDeviceName.value
    )
  }
}

function onDeviceChange(event: Event) {
  console.log('>> onDeviceChange', event)
  if (!furHatDebounce.value) {
    furHatDebounce.value = true
    setTimeout(() => {
      // It takes a couple of seconds to OS to properly update the devices
      updateDevices()
      if (oldDeviceCount.value < outputDeviceCount.value) {
        // Device was removed
        console.warn('device was REMOVED -- no op')
        // setAudioOutput(systemDefaultAudioDeviceName)
      } else {
        // Device was added
        console.warn('device was maybe SoundADDED')
        getDefaultOutput()
        console.warn('currentOuputDeviceName: ', currentOuputDeviceName.value)
        if (currentOuputDeviceName.value !== preferredAudioDeviceName) {
          setAudioOutput(preferredAudioDeviceName)
        }
        setTimeout(() => {
          furHatDebounce.value = false
        }, 10)
      }
    }, 200)
  } else {
    console.log('>> onDeviceChange debounced')
  }
}

onMounted(async () => {
  await navigator.mediaDevices.getUserMedia({ video: false, audio: true })
  updateDevices()
  getDefaultOutput()
  navigator.mediaDevices.ondevicechange = (event) => {
    onDeviceChange(event)
  }
})

setTimeout(() => {
  initialDelayDone.value = true
}, 2000)
</script>

<template>
  <div class="flex flex-col p-2">
    <div>
      <h3>Output devices</h3>

      <ul class="py-4 list-disc list-inside">
        <li v-for="device in outputDevices" :key="device">
          {{ device }}
          <span v-if="currentOuputDeviceName === device"> (selected) </span>
        </li>
      </ul>
    </div>
  </div>
</template>
