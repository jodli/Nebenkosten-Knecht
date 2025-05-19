<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold text-gray-800">
        Tenants
      </h1>
      <router-link
        to="/tenants/create"
        class="btn btn-primary"
      >
        Add Tenant
      </router-link>
    </div>

    <div
      v-if="loading"
      class="text-center py-8"
    >
      <p class="text-gray-600">
        Loading tenants...
      </p>
    </div>

    <div
      v-else-if="error"
      class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded"
    >
      {{ error }}
    </div>

    <div
      v-else-if="tenants.length === 0"
      class="card text-center py-8"
    >
      <p class="text-gray-600">
        No tenants found. Create your first tenant to get started.
      </p>
      <div class="mt-4">
        <router-link
          to="/tenants/create"
          class="btn btn-primary"
        >
          Add Tenant
        </router-link>
      </div>
    </div>

    <div
      v-else
      class="grid grid-cols-1 gap-4"
    >
      <div
        v-for="tenant in tenants"
        :key="tenant.id"
        class="card hover:shadow-lg transition-shadow"
      >
        <div class="flex justify-between items-start">
          <div>
            <h2 class="text-xl font-semibold text-gray-800">
              {{ tenant.name }}
            </h2>
            <p class="text-gray-600 mt-1">
              Number of Persons: {{ tenant.number_of_persons }}
            </p>
            <p class="text-gray-600 mt-1">
              Property Unit: {{ getPropertyUnitName(tenant.property_unit_id) }}
            </p>
          </div>
          <div class="flex space-x-2">
            <router-link
              :to="`/tenants/${tenant.id}`"
              class="btn btn-secondary"
            >
              Edit
            </router-link>
            <button
              class="btn btn-danger"
              @click="confirmDelete(tenant)"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal (simplified for MVP) -->
    <div
      v-if="showDeleteModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-white rounded-lg p-6 max-w-md mx-auto">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          Confirm Deletion
        </h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the tenant "{{ tenantToDelete?.name }}"?
          This action cannot be undone.
        </p>
        <div class="flex justify-end space-x-3">
          <button
            class="btn btn-secondary"
            @click="showDeleteModal = false"
          >
            Cancel
          </button>
          <button
            class="btn btn-danger"
            @click="deleteTenant"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { tenantService, propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'TenantList',
  setup() {
    const toast = useToast();
    return { toast };
  },
  data() {
    return {
      tenants: [],
      propertyUnits: [],
      loading: true,
      error: null,
      showDeleteModal: false,
      tenantToDelete: null
    };
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      try {
        this.loading = true;

        // Fetch both tenants and property units in parallel
        const [tenantsResponse, propertyUnitsResponse] = await Promise.all([
          tenantService.getAll(),
          propertyUnitService.getAll()
        ]);

        this.tenants = tenantsResponse.data;
        this.propertyUnits = propertyUnitsResponse.data;

        this.loading = false;
      } catch (error) {
        this.error = 'Failed to load tenants';
        this.toast.error(this.error);
        this.loading = false;
        console.error('Error fetching data:', error);
      }
    },
    getPropertyUnitName(propertyUnitId) {
      const unit = this.propertyUnits.find(unit => unit.id === propertyUnitId);
      return unit ? unit.name : 'Unknown';
    },
    confirmDelete(tenant) {
      this.tenantToDelete = tenant;
      this.showDeleteModal = true;
    },
    async deleteTenant() {
      if (!this.tenantToDelete) return;

      try {
        await tenantService.delete(this.tenantToDelete.id);
        this.toast.success(`Tenant "${this.tenantToDelete.name}" deleted successfully`);

        // Remove from local list
        this.tenants = this.tenants.filter(
          tenant => tenant.id !== this.tenantToDelete.id
        );

        // Close modal
        this.showDeleteModal = false;
        this.tenantToDelete = null;
      } catch (error) {
        this.toast.error('Failed to delete tenant');
        console.error('Error deleting tenant:', error);
      }
    }
  }
};
</script>
