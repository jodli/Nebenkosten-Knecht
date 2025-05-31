<template>
  <div>
    <PageHeader title="Tenants">
      <template #actions>
        <BaseButton
          @click="$router.push('/tenants/create')"
          variant="primary"
        >
          Add Tenant
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading tenants..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <EmptyState
      v-else-if="tenants.length === 0"
      title="No Tenants Found"
      message="Create your first tenant to get started."
    >
      <template #actions>
        <BaseButton
          @click="$router.push('/tenants/create')"
          variant="primary"
        >
          Add Tenant
        </BaseButton>
      </template>
    </EmptyState>

    <div v-else class="card-grid">
      <DataCard
        v-for="tenant in tenants"
        :key="tenant.id"
        :title="tenant.name"
      >
        <div class="space-y-2">
          <p class="text-gray-600">
            <span class="font-medium">Persons:</span> {{ tenant.number_of_persons }}
          </p>
          <p class="text-gray-600">
            <span class="font-medium">Property Unit:</span> {{ getPropertyUnitName(tenant.property_unit_id) }}
          </p>
        </div>

        <template #actions>
          <BaseButton
            size="sm"
            variant="secondary"
            @click="$router.push(`/tenants/${tenant.id}`)"
          >
            Edit
          </BaseButton>
          <BaseButton
            size="sm"
            variant="danger"
            @click="confirmDelete(tenant)"
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
          Are you sure you want to delete the tenant "{{ tenantToDelete?.name }}"?
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
            @click="deleteTenant"
          >
            Delete
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { tenantService, propertyUnitService } from '@/services/api';
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
  name: 'TenantList',
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
