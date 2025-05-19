<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold text-gray-800">
        Property Units
      </h1>
      <router-link
        to="/property-units/create"
        class="btn btn-primary"
      >
        Add Property Unit
      </router-link>
    </div>

    <div
      v-if="loading"
      class="text-center py-8"
    >
      <p class="text-gray-600">
        Loading property units...
      </p>
    </div>

    <div
      v-else-if="error"
      class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded"
    >
      {{ error }}
    </div>

    <div
      v-else-if="propertyUnits.length === 0"
      class="card text-center py-8"
    >
      <p class="text-gray-600">
        No property units found. Create your first property unit to get started.
      </p>
      <div class="mt-4">
        <router-link
          to="/property-units/create"
          class="btn btn-primary"
        >
          Add Property Unit
        </router-link>
      </div>
    </div>

    <div
      v-else
      class="grid grid-cols-1 gap-4"
    >
      <div
        v-for="unit in propertyUnits"
        :key="unit.id"
        class="card hover:shadow-lg transition-shadow"
      >
        <div class="flex justify-between items-start">
          <div>
            <h2 class="text-xl font-semibold text-gray-800">
              {{ unit.name }}
            </h2>
            <p class="text-gray-600 mt-1">
              Living Area: {{ unit.living_area_m2 }} m²
            </p>
          </div>
          <div class="flex space-x-2">
            <router-link
              :to="`/property-units/${unit.id}`"
              class="btn btn-secondary"
            >
              Edit
            </router-link>
            <button
              class="btn btn-danger"
              @click="confirmDelete(unit)"
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
          Are you sure you want to delete the property unit "{{ unitToDelete?.name }}"?
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
            @click="deletePropertyUnit"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'PropertyUnitList',
  setup() {
    const toast = useToast();
    return { toast };
  },
  data() {
    return {
      propertyUnits: [],
      loading: true,
      error: null,
      showDeleteModal: false,
      unitToDelete: null
    };
  },
  mounted() {
    this.fetchPropertyUnits();
  },
  methods: {
    async fetchPropertyUnits() {
      try {
        this.loading = true;
        const response = await propertyUnitService.getAll();
        this.propertyUnits = response.data;
        this.loading = false;
      } catch (error) {
        this.error = 'Failed to load property units';
        this.toast.error(this.error);
        this.loading = false;
        console.error('Error fetching property units:', error);
      }
    },
    confirmDelete(unit) {
      this.unitToDelete = unit;
      this.showDeleteModal = true;
    },
    async deletePropertyUnit() {
      if (!this.unitToDelete) return;

      try {
        await propertyUnitService.delete(this.unitToDelete.id);
        this.toast.success(`Property unit "${this.unitToDelete.name}" deleted successfully`);

        // Remove from local list
        this.propertyUnits = this.propertyUnits.filter(
          unit => unit.id !== this.unitToDelete.id
        );

        // Close modal
        this.showDeleteModal = false;
        this.unitToDelete = null;
      } catch (error) {
        this.toast.error('Failed to delete property unit');
        console.error('Error deleting property unit:', error);
      }
    }
  }
};
</script>
