<template>
  <nav class="bg-primary-700 shadow-md">
    <div class="container mx-auto px-4">
      <div class="flex justify-between h-16">
        <div class="flex">
          <div class="flex-shrink-0 flex items-center">
            <router-link to="/" class="text-white text-xl font-semibold">
              Nebenkosten-Knecht
            </router-link>
          </div>
          <div class="hidden md:ml-6 md:flex md:items-center md:space-x-4">
            <router-link to="/" class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              exact-active-class="bg-primary-800">
              Dashboard
            </router-link>
            <router-link to="/property-units"
              class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              active-class="bg-primary-800">
              Property Units
            </router-link>
            <router-link to="/tenants" class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              active-class="bg-primary-800">
              Tenants
            </router-link>
            <router-link to="/meters" class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              active-class="bg-primary-800">
              Meters
            </router-link>
            <router-link to="/meter-readings"
              class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              active-class="bg-primary-800">
              Meter Readings
            </router-link>
            <router-link to="/cost-types"
              class="px-3 py-2 rounded-md text-sm font-medium text-white hover:bg-primary-600"
              active-class="bg-primary-800">
              Cost Types
            </router-link>
          </div>
        </div>

        <!-- Mobile menu button -->
        <div class="flex items-center md:hidden">
          <button
            class="inline-flex items-center justify-center p-2 rounded-md text-white hover:bg-primary-600 focus:outline-none"
            @click="mobileMenuOpen = !mobileMenuOpen">
            <span class="sr-only">Open main menu</span>
            <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path v-if="!mobileMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M4 6h16M4 12h16m-7 6h7" />
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Mobile menu -->
    <div v-if="mobileMenuOpen" class="md:hidden">
      <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3">
        <router-link to="/" class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          exact-active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Dashboard
        </router-link>
        <router-link to="/property-units"
          class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Property Units
        </router-link>
        <router-link to="/tenants"
          class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Tenants
        </router-link>
        <router-link to="/meters"
          class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Meters
        </router-link>
        <router-link to="/meter-readings"
          class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Meter Readings
        </router-link>
        <router-link to="/cost-types"
          class="block px-3 py-2 rounded-md text-base font-medium text-white hover:bg-primary-600"
          active-class="bg-primary-800" @click="mobileMenuOpen = false">
          Cost Types
        </router-link>
      </div>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'NavBar',
  data() {
    return {
      mobileMenuOpen: false
    };
  }
};
</script>
