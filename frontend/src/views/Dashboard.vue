<template>
  <div>
    <h1 class="text-3xl font-bold mb-8 text-gray-800">
      Welcome to Nebenkosten-Knecht
    </h1>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Property Units Card -->
      <div class="card bg-white hover:shadow-lg transition-shadow">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-gray-800">
            Property Units
          </h2>
          <span class="text-2xl font-bold text-primary-600">{{ propertyUnitCount }}</span>
        </div>
        <p class="text-gray-600 mt-2">
          Manage your properties and their living areas
        </p>
        <div class="mt-4">
          <router-link to="/property-units" class="btn btn-primary">
            Manage Property Units
          </router-link>
        </div>
      </div>

      <!-- Tenants Card -->
      <div class="card bg-white hover:shadow-lg transition-shadow">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-gray-800">
            Tenants
          </h2>
          <span class="text-2xl font-bold text-primary-600">{{ tenantCount }}</span>
        </div>
        <p class="text-gray-600 mt-2">
          Manage your tenants and their assignment to properties
        </p>
        <div class="mt-4">
          <router-link to="/tenants" class="btn btn-primary">
            Manage Tenants
          </router-link>
        </div>
      </div>

      <!-- Meters Card -->
      <div class="card bg-white hover:shadow-lg transition-shadow">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-gray-800">
            Meters
          </h2>
          <span class="text-2xl font-bold text-primary-600">{{ meterCount }}</span>
        </div>
        <p class="text-gray-600 mt-2">
          Manage your utility meters and their assignment
        </p>
        <div class="mt-4">
          <router-link to="/meters" class="btn btn-primary">
            Manage Meters
          </router-link>
        </div>
      </div>

      <!-- Meter Readings Card -->
      <div class="card bg-white hover:shadow-lg transition-shadow">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-gray-800">
            Meter Readings
          </h2>
          <span class="text-2xl font-bold text-primary-600">{{ readingCount }}</span>
        </div>
        <p class="text-gray-600 mt-2">
          Track meter readings and view consumption
        </p>
        <div class="mt-4">
          <router-link to="/meter-readings" class="btn btn-primary">
            Manage Readings
          </router-link>
        </div>
      </div>
    </div>

    <div class="mt-12">
      <h2 class="text-2xl font-semibold mb-4 text-gray-800">
        Getting Started
      </h2>
      <div class="card">
        <ol class="list-decimal list-inside space-y-3 text-gray-700">
          <li>
            <strong>Create Property Units</strong> - Define the areas of your property
            (e.g., main apartment, tenant apartment)
          </li>
          <li>
            <strong>Add Tenants</strong> - Create tenants and assign them to your property units
          </li>
          <li>
            <strong>Set Up Meters</strong> - Define all utility meters, whether specific to a unit or common
          </li>
          <li>
            <strong>Record Meter Readings</strong> - Regularly enter readings for your meters and track consumption
          </li>
        </ol>
      </div>
    </div>
  </div>
</template>

<script>
import { propertyUnitService, tenantService, meterService, meterReadingService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'DashboardView',
  setup() {
    const toast = useToast();
    return { toast };
  },
  data() {
    return {
      propertyUnitCount: 0,
      tenantCount: 0,
      meterCount: 0,
      readingCount: 0,
      loading: true,
      error: null
    };
  },
  mounted() {
    this.fetchCounts();
  },
  methods: {
    async fetchCounts() {
      try {
        this.loading = true;

        // Fetch property units
        const propertyUnitsResponse = await propertyUnitService.getAll();
        this.propertyUnitCount = propertyUnitsResponse.data.length;

        // Fetch tenants
        const tenantsResponse = await tenantService.getAll();
        this.tenantCount = tenantsResponse.data.length;

        // Fetch meters
        const metersResponse = await meterService.getAll();
        this.meterCount = metersResponse.data.length;

        // Fetch meter readings
        const readingsResponse = await meterReadingService.getAll();
        this.readingCount = readingsResponse.data.length;

        this.loading = false;
      } catch (error) {
        this.error = 'Failed to load dashboard data';
        this.toast.error(this.error);
        this.loading = false;
        console.error('Error fetching dashboard data:', error);
      }
    }
  }
};
</script>
