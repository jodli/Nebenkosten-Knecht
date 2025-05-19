<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold text-gray-800">
        Meters
      </h1>
      <router-link
        to="/meters/create"
        class="btn btn-primary"
      >
        Add Meter
      </router-link>
    </div>

    <div
      v-if="loading"
      class="text-center py-8"
    >
      <p class="text-gray-600">
        Loading meters...
      </p>
    </div>

    <div
      v-else-if="error"
      class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded"
    >
      {{ error }}
    </div>

    <div
      v-else-if="meters.length === 0"
      class="card text-center py-8"
    >
      <p class="text-gray-600">
        No meters found. Create your first meter to get started.
      </p>
      <div class="mt-4">
        <router-link
          to="/meters/create"
          class="btn btn-primary"
        >
          Add Meter
        </router-link>
      </div>
    </div>

    <div v-else>
      <!-- Common Meters Section -->
      <div
        v-if="commonMeters.length > 0"
        class="mb-8"
      >
        <h2 class="text-xl font-semibold text-gray-800 mb-4">
          Common Meters
        </h2>
        <div class="grid grid-cols-1 gap-4">
          <div
            v-for="meter in commonMeters"
            :key="meter.id"
            class="card hover:shadow-lg transition-shadow"
          >
            <div class="flex justify-between items-start">
              <div>
                <h3 class="text-lg font-semibold text-gray-800">
                  {{ meter.name }}
                </h3>
                <p class="text-gray-600 mt-1">
                  Type: {{ meter.meter_type }}
                </p>
                <p class="text-gray-600">
                  Unit: {{ meter.unit }}
                </p>
                <p class="text-gray-600">
                  Assignment: Common
                </p>
              </div>
              <div class="flex space-x-2">
                <router-link
                  :to="`/meters/${meter.id}`"
                  class="btn btn-secondary"
                >
                  Edit
                </router-link>
                <button
                  class="btn btn-danger"
                  @click="confirmDelete(meter)"
                >
                  Delete
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Unit Meters Section -->
      <div v-if="unitMeters.length > 0">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">
          Unit Meters
        </h2>
        <div class="grid grid-cols-1 gap-4">
          <div
            v-for="meter in unitMeters"
            :key="meter.id"
            class="card hover:shadow-lg transition-shadow"
          >
            <div class="flex justify-between items-start">
              <div>
                <h3 class="text-lg font-semibold text-gray-800">
                  {{ meter.name }}
                </h3>
                <p class="text-gray-600 mt-1">
                  Type: {{ meter.meter_type }}
                </p>
                <p class="text-gray-600">
                  Unit: {{ meter.unit }}
                </p>
                <p class="text-gray-600">
                  Assignment: {{ getPropertyUnitName(meter.property_unit_id) }}
                </p>
              </div>
              <div class="flex space-x-2">
                <router-link
                  :to="`/meters/${meter.id}`"
                  class="btn btn-secondary"
                >
                  Edit
                </router-link>
                <button
                  class="btn btn-danger"
                  @click="confirmDelete(meter)"
                >
                  Delete
                </button>
              </div>
            </div>
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
          Are you sure you want to delete the meter "{{ meterToDelete?.name }}"?
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
            @click="deleteMeter"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { meterService, propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'MeterList',
  setup() {
    const toast = useToast();
    return { toast };
  },
  data() {
    return {
      meters: [],
      propertyUnits: [],
      loading: true,
      error: null,
      showDeleteModal: false,
      meterToDelete: null
    };
  },
  computed: {
    commonMeters() {
      return this.meters.filter(meter => meter.assignment_type === 'common');
    },
    unitMeters() {
      return this.meters.filter(meter => meter.assignment_type === 'unit');
    }
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      try {
        this.loading = true;

        // Fetch both meters and property units in parallel
        const [metersResponse, propertyUnitsResponse] = await Promise.all([
          meterService.getAll(),
          propertyUnitService.getAll()
        ]);

        this.meters = metersResponse.data;
        this.propertyUnits = propertyUnitsResponse.data;

        this.loading = false;
      } catch (error) {
        this.error = 'Failed to load meters';
        this.toast.error(this.error);
        this.loading = false;
        console.error('Error fetching data:', error);
      }
    },
    getPropertyUnitName(propertyUnitId) {
      if (!propertyUnitId) return 'Common';
      const unit = this.propertyUnits.find(unit => unit.id === propertyUnitId);
      return unit ? unit.name : 'Unknown';
    },
    confirmDelete(meter) {
      this.meterToDelete = meter;
      this.showDeleteModal = true;
    },
    async deleteMeter() {
      if (!this.meterToDelete) return;

      try {
        await meterService.delete(this.meterToDelete.id);
        this.toast.success(`Meter "${this.meterToDelete.name}" deleted successfully`);

        // Remove from local list
        this.meters = this.meters.filter(
          meter => meter.id !== this.meterToDelete.id
        );

        // Close modal
        this.showDeleteModal = false;
        this.meterToDelete = null;
      } catch (error) {
        this.toast.error('Failed to delete meter');
        console.error('Error deleting meter:', error);
      }
    }
  }
};
</script>
