import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'
import { getCurrent } from '@tauri-apps/api/window'
import { Store } from 'tauri-plugin-store-api'

export const useStore = defineStore('store', () => {
  const appSettings = new Store('autoselect-settings.json')
  const currentPage = ref<'splash' | 'main' | 'settings'>('splash')
  const setupDone = ref(false)
  const systemDefaultAudioDeviceName = ref('')
  const preferredAudioDeviceName = ref('')
  const currentOuputDeviceName = ref('')
  const initialDelayDone = ref(false)
  const outputDevices = ref<string[]>([])
  const oldDeviceCount = ref(0)
  const outputDeviceCount = computed(() => outputDevices.value.length)
  const furHatDebounce = ref(false)
  const isInited = ref(false)

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
          if (currentOuputDeviceName.value !== preferredAudioDeviceName.value) {
            setAudioOutput(preferredAudioDeviceName.value)
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

  function quitApp() {
    const window = getCurrent()
    window?.close()
  }

  async function loadSettings() {
    setupDone.value = (await appSettings.get<boolean>('setupDone')) ?? false

    systemDefaultAudioDeviceName.value =
      (await appSettings.get('systemDefaultAudioDeviceName')) ?? 'MacBook Pro Speakers'
    preferredAudioDeviceName.value =
      (await appSettings.get('preferredAudioDeviceName')) ?? 'Beats Fit Pro'
  }

  async function saveSettings() {
    await appSettings.set('setupDone', true)
    await appSettings.set('systemDefaultAudioDeviceName', systemDefaultAudioDeviceName.value)
    await appSettings.set('preferredAudioDeviceName', preferredAudioDeviceName.value)
    await appSettings.save()
    currentPage.value = 'main'
  }

  function setPage(page: 'splash' | 'main' | 'settings') {
    currentPage.value = page
  }

  async function init() {
    console.log('Initing app')
    loadSettings()

    const window = getCurrent()

    currentOuputDeviceName.value = systemDefaultAudioDeviceName.value

    // FIXME: this doesnt work w/o open Wry Window, hence the DIY flash screen
    await navigator.mediaDevices.getUserMedia({ video: false, audio: true })
    window?.hide()

    updateDevices()
    getDefaultOutput()
    navigator.mediaDevices.ondevicechange = (event) => {
      onDeviceChange(event)
    }

    setTimeout(() => {
      initialDelayDone.value = true
      isInited.value = true
      if (!setupDone.value) {
        currentPage.value = 'settings'
      } else {
        // currentPage.value = 'settings'
        currentPage.value = 'main'
      }

      console.log('initial delay done')
    }, 2000)
    console.log('App inited')
  }

  return {
    init,
    isInited,
    outputDevices,
    currentOuputDeviceName,
    initialDelayDone,
    quitApp,
    currentPage,
    saveSettings,
    setPage,
    systemDefaultAudioDeviceName,
    preferredAudioDeviceName,
    setupDone,
  }
})
