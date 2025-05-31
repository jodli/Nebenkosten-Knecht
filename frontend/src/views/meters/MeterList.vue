<template>
  <div>
    <PageHeader title="Meters">
      <template #actions>
        <BaseButton
          @click="$router.push('/meters/create')"
          variant="primary"
        >
          Add Meter
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading meters..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <EmptyState
      v-else-if="meters.length === 0"
      title="No Meters Found"
      message="Create your first meter to get started."
    >
      <template #actions>
        <BaseButton
          @click="$router.push('/meters/create')"
          variant="primary"
        >
          Add Meter
        </BaseButton>
      </template>
    </EmptyState>

    <div v-else>
      <!-- Common Meters Section -->
      <div
        v-if="commonMeters.length > 0"
        class="mb-8"
      >
        <h2 class="section-title">
          Common Meters
        </h2>
        <div class="card-grid">
          <DataCard
            v-for="meter in commonMeters"
            :key="meter.id"
            :title="meter.name"
          >
            <div class="space-y-1">
              <p class="text-gray-600">
                <span class="font-medium">Type:</span> {{ meter.meter_type }}
              </p>
              <p class="text-gray-600">
                <span class="font-medium">Unit:</span> {{ meter.unit }}
              </p>
              <p class="text-gray-600">
                <span class="font-medium">Assignment:</span> Common
              </p>
            </div>

            <template #actions>
              <BaseButton
                size="sm"
                variant="secondary"
                @click="$router.push(`/meters/${meter.id}`)"
              >
                Edit
              </BaseButton>
              <BaseButton
                size="sm"
                variant="danger"
                @click="confirmDelete(meter)"
              >
                Delete
              </BaseButton>
            </template>
          </DataCard>
        </div>
      </div>

      <!-- Unit Meters Section -->
      <div v-if="unitMeters.length > 0">
        <h2 class="section-title">
          Unit Meters
        </h2>
        <div class="card-grid">
          <DataCard
            v-for="meter in unitMeters"
            :key="meter.id"
            :title="meter.name"
          >
            <div class="space-y-1">
              <p class="text-gray-600">
                <span class="font-medium">Type:</span> {{ meter.meter_type }}
              </p>
              <p class="text-gray-600">
                <span class="font-medium">Unit:</span> {{ meter.unit }}
              </p>
              <p class="text-gray-600">
                <span class="font-medium">Property Unit:</span> {{ getPropertyUnitName(meter.property_unit_id) }}
              </p>
            </div>

            <template #actions>
              <BaseButton
                size="sm"
                variant="secondary"
                @click="$router.push(`/meters/${meter.id}`)"
              >
                Edit
              </BaseButton>
              <BaseButton
                size="sm"
                variant="danger"
                @click="confirmDelete(meter)"
              >
                Delete
              </BaseButton>
            </template>
          </DataCard>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div
      v-if="showDeleteModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <BaseCard class="max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          Confirm Deletion
        </h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the meter "{{ meterToDelete?.name }}"?
          This action cannot be undone.
        </p>
        <div class="flex justify-end space-x-3">
          <BaseButton
            variant="secondary"
            @click="showDeleteModal = false"
          >
            Cancel
          </BaseButton>
          <BaseButton
            variant="danger"
            @click="deleteMeter"
          >
            Delete
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { meterService, propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  DataCard,
  LoadingState,
  AlertState,
  EmptyState
} from '@/components/base';

export default {
  name: 'MeterList',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    DataCard,
    LoadingState,
    AlertState,
    EmptyState
  },
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
