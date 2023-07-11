<script setup lang="ts">
import SplashScreen from '@/pages/SplashScreen.vue'
import MainPage from '@/pages/MainPage.vue'
import SettingsPage from '@/pages/SettingsPage.vue'

const store = useStore()
const isSplashScreen = computed(() => store.currentPage === 'splash')
</script>

<template>
  <div class="flex flex-col h-full px-4 pt-4 rounded-md" :class="{ splash: isSplashScreen }">
    <SplashScreen />
    <MainPage />
    <SettingsPage />
  </div>
</template>
