<template>
  <div>
    <PageHeader title="Cost Types">
      <template #actions>
        <BaseButton
          @click="$router.push('/cost-types/create')"
          variant="primary"
        >
          Add Cost Type
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading cost types..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      title="Error"
      :message="error"
    />

    <EmptyState
      v-else-if="!costTypes.length"
      title="No Cost Types Found"
      message="Create your first cost type to get started."
    >
      <template #actions>
        <BaseButton
          @click="$router.push('/cost-types/create')"
          variant="primary"
        >
          Add Cost Type
        </BaseButton>
      </template>
    </EmptyState>

    <BaseTable
      v-else
      :columns="tableHeaders"
      :data="costTypes"
      :actions="tableActions"
      @action="handleTableAction"
    >
      <template #cell-name="{ item }">
        <div class="font-medium text-gray-900">{{ item.name }}</div>
        <div v-if="item.description" class="text-sm text-gray-500">
          {{ item.description }}
        </div>
      </template>

      <template #cell-type="{ item }">
        <span
          :class="[
            'inline-flex px-2 py-1 text-xs font-semibold rounded-full',
            item.is_consumption_based
              ? 'bg-green-100 text-green-800'
              : 'bg-blue-100 text-blue-800'
          ]"
        >
          {{ item.is_consumption_based ? 'Consumption' : 'Fixed' }}
        </span>
      </template>

      <template #cell-unit="{ item }">
        <span class="text-gray-600">
          {{ item.unit || '-' }}
        </span>
      </template>

      <template #cell-allocation_methods="{ item }">
        <div class="flex flex-wrap gap-1">
          <span
            v-for="method in item.allocation_methods"
            :key="method.id"
            class="inline-flex px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800 rounded"
          >
            {{ method.name }}
          </span>
          <span v-if="!item.allocation_methods.length" class="text-gray-500">
            None
          </span>
        </div>
      </template>
    </BaseTable>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="modal-overlay">
      <BaseCard class="max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          Confirm Deletion
        </h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the cost type "{{ costTypeToDelete.name }}"? This action cannot be undone.
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
            @click="deleteCostType"
          >
            Delete
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { costTypeService } from '@/services/api';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  BaseTable,
  LoadingState,
  AlertState,
  EmptyState
} from '@/components/base';

export default {
  name: 'CostTypeList',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    BaseTable,
    LoadingState,
    AlertState,
    EmptyState
  },
  setup() {
    const costTypes = ref([]);
    const loading = ref(true);
    const error = ref(null);
    const showDeleteModal = ref(false);
    const costTypeToDelete = ref({});

    const tableHeaders = [
      { key: 'name', label: 'Name' },
      { key: 'type', label: 'Type' },
      { key: 'unit', label: 'Unit' },
      { key: 'allocation_methods', label: 'Allocation Methods' }
    ];

    const tableActions = [
      { key: 'edit', label: 'Edit', variant: 'secondary' },
      { key: 'tariffs', label: 'Tariffs', variant: 'secondary', condition: (item) => item.is_consumption_based },
      { key: 'fixed-costs', label: 'Fixed Costs', variant: 'secondary', condition: (item) => !item.is_consumption_based },
      { key: 'delete', label: 'Delete', variant: 'danger' }
    ];

    const handleTableAction = (action, item) => {
      switch (action) {
        case 'edit':
          // Router push would be handled in parent component or via emits
          window.location.href = `/cost-types/${item.id}/edit`;
          break;
        case 'tariffs':
          window.location.href = `/cost-types/${item.id}/tariffs`;
          break;
        case 'fixed-costs':
          window.location.href = `/cost-types/${item.id}/fixed-costs`;
          break;
        case 'delete':
          confirmDelete(item);
          break;
      }
    };

    const fetchCostTypes = async () => {
      loading.value = true;
      error.value = null;
      try {
        const response = await costTypeService.getAll();
        costTypes.value = response.data;
      } catch (err) {
        console.error('Error fetching cost types:', err);
        error.value = err.response?.data || 'An error occurred while fetching cost types.';
      } finally {
        loading.value = false;
      }
    };

    const confirmDelete = (costType) => {
      costTypeToDelete.value = costType;
      showDeleteModal.value = true;
    };

    const deleteCostType = async () => {
      try {
        await costTypeService.delete(costTypeToDelete.value.id);
        showDeleteModal.value = false;
        fetchCostTypes();
      } catch (err) {
        console.error('Error deleting cost type:', err);
        error.value = err.response?.data || 'An error occurred while deleting the cost type.';
        showDeleteModal.value = false;
      }
    };

    onMounted(fetchCostTypes);

    return {
      costTypes,
      loading,
      error,
      showDeleteModal,
      costTypeToDelete,
      tableHeaders,
      tableActions,
      handleTableAction,
      confirmDelete,
      deleteCostType
    };
  }
};
</script>
