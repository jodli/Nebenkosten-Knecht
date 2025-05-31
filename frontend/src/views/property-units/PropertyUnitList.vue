<template>
  <div>
    <PageHeader title="Property Units">
      <template #actions>
        <BaseButton
          @click="$router.push('/property-units/create')"
          variant="primary"
        >
          Add Property Unit
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading property units..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <EmptyState
      v-else-if="propertyUnits.length === 0"
      title="No Property Units Found"
      message="Create your first property unit to get started."
    >
      <template #actions>
        <BaseButton
          @click="$router.push('/property-units/create')"
          variant="primary"
        >
          Add Property Unit
        </BaseButton>
      </template>
    </EmptyState>

    <div v-else class="card-grid"
    >
      <DataCard
        v-for="unit in propertyUnits"
        :key="unit.id"
        :title="unit.name"
      >
        <p class="text-gray-600">
          <span class="font-medium">Living Area:</span> {{ unit.living_area_m2 }} m²
        </p>

        <template #actions>
          <BaseButton
            size="sm"
            variant="secondary"
            @click="$router.push(`/property-units/${unit.id}`)"
          >
            Edit
          </BaseButton>
          <BaseButton
            size="sm"
            variant="danger"
            @click="confirmDelete(unit)"
          >
            Delete
          </BaseButton>
        </template>
      </DataCard>
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
          Are you sure you want to delete the property unit "{{ unitToDelete?.name }}"?
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
            @click="deletePropertyUnit"
          >
            Delete
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { propertyUnitService } from '@/services/api';
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
  name: 'PropertyUnitList',
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
