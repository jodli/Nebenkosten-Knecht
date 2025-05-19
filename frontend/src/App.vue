<template>
  <div class="min-h-screen bg-gray-100">
    <Navbar />
    <div class="container mx-auto px-4 py-8">
      <router-view />
    </div>
  </div>
</template>

<script>
import Navbar from '@/components/Navbar.vue';

export default {
  name: 'App',
  components: {
    Navbar
  }
};
</script>
