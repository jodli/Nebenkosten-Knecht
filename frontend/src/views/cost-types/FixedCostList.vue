<template>
  <div>
    <PageHeader :title="`Fixed Costs for ${costType.name}`">
      <template #subtitle v-if="costType.description">
        {{ costType.description }}
      </template>
      <template #actions>
        <BaseButton
          @click="$router.push('/cost-types')"
          variant="secondary"
        >
          Back to List
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading fixed costs..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <div v-else class="space-y-6">
      <!-- Add Fixed Cost Form -->
      <BaseCard>
        <h2 class="text-xl font-semibold text-gray-800 mb-6">Add New Fixed Cost</h2>
        <form @submit.prevent="addFixedCost" class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label class="form-label" for="amount">
                Amount <span class="text-red-500">*</span>
              </label>
              <input
                id="amount"
                v-model.number="newFixedCost.amount"
                type="number"
                step="0.01"
                min="0.01"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="billing-period-start">
                Billing Period Start <span class="text-red-500">*</span>
              </label>
              <input
                id="billing-period-start"
                v-model="newFixedCost.billing_period_start"
                type="date"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="billing-period-end">
                Billing Period End <span class="text-red-500">*</span>
              </label>
              <input
                id="billing-period-end"
                v-model="newFixedCost.billing_period_end"
                type="date"
                class="form-input"
                required
              />
            </div>
          </div>
          <div class="flex justify-end">
            <BaseButton
              type="submit"
              variant="primary"
              :disabled="submitting"
            >
              {{ submitting ? 'Adding...' : 'Add Fixed Cost' }}
            </BaseButton>
          </div>
        </form>
      </BaseCard>

      <!-- Fixed Costs List -->
      <EmptyState
        v-if="!fixedCosts.length"
        title="No Fixed Costs Found"
        message="No fixed costs found for this cost type."
      />

      <BaseTable
        v-else
        :columns="tableHeaders"
        :data="fixedCosts"
        :actions="tableActions"
        @action="handleTableAction"
      >
        <template #cell-amount="{ item }">
          €{{ item.amount.toFixed(2) }}
        </template>
        <template #cell-billing_period="{ item }">
          {{ formatDate(item.billing_period_start) }} - {{ formatDate(item.billing_period_end) }}
        </template>
      </BaseTable>

      <!-- Edit Fixed Cost Modal -->
      <div v-if="showEditModal" class="modal-overlay">
        <BaseCard class="max-w-md mx-4">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">Edit Fixed Cost</h3>
          <form @submit.prevent="updateFixedCost" class="space-y-4">
            <div>
              <label class="form-label" for="edit-amount">
                Amount <span class="text-red-500">*</span>
              </label>
              <input
                id="edit-amount"
                v-model.number="editingFixedCost.amount"
                type="number"
                step="0.01"
                min="0.01"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="edit-billing-period-start">
                Billing Period Start <span class="text-red-500">*</span>
              </label>
              <input
                id="edit-billing-period-start"
                v-model="editingFixedCost.billing_period_start"
                type="date"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="edit-billing-period-end">
                Billing Period End <span class="text-red-500">*</span>
              </label>
              <input
                id="edit-billing-period-end"
                v-model="editingFixedCost.billing_period_end"
                type="date"
                class="form-input"
                required
              />
            </div>
            <div class="flex justify-end space-x-3">
              <BaseButton
                type="button"
                @click="showEditModal = false"
                variant="secondary"
              >
                Cancel
              </BaseButton>
              <BaseButton
                type="submit"
                variant="primary"
                :disabled="submitting"
              >
                {{ submitting ? 'Saving...' : 'Save Changes' }}
              </BaseButton>
            </div>
          </form>
        </BaseCard>
      </div>

      <!-- Delete Confirmation Modal -->
      <div v-if="showDeleteModal" class="modal-overlay">
        <BaseCard class="max-w-md mx-4">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">
            Confirm Deletion
          </h3>
          <p class="text-gray-600 mb-6">
            Are you sure you want to delete the fixed cost of €{{ fixedCostToDelete.amount?.toFixed(2) }}
            for period {{ formatDate(fixedCostToDelete.billing_period_start) }} - {{ formatDate(fixedCostToDelete.billing_period_end) }}?
            This action cannot be undone.
          </p>
          <div class="flex justify-end space-x-3">
            <BaseButton
              @click="showDeleteModal = false"
              variant="secondary"
            >
              Cancel
            </BaseButton>
            <BaseButton
              @click="deleteFixedCost"
              variant="danger"
              :disabled="submitting"
            >
              {{ submitting ? 'Deleting...' : 'Delete' }}
            </BaseButton>
          </div>
        </BaseCard>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { costTypeService, fixedCostService } from '@/services/api';
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
  name: 'FixedCostList',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    BaseTable,
    LoadingState,
    AlertState,
    EmptyState
  },
  props: {
    id: {
      type: String,
      required: true
    }
  },
  setup(props) {
    const costTypeId = parseInt(props.id);

    const costType = ref({});
    const fixedCosts = ref([]);
    const loading = ref(true);
    const submitting = ref(false);
    const error = ref(null);

    // Table configuration
    const tableHeaders = [
      { key: 'amount', label: 'Amount' },
      { key: 'billing_period', label: 'Billing Period' }
    ];

    const tableActions = [
      { key: 'edit', label: 'Edit', variant: 'secondary' },
      { key: 'delete', label: 'Delete', variant: 'danger' }
    ];

    const handleTableAction = (action, item) => {
      switch (action) {
        case 'edit':
          editFixedCost(item);
          break;
        case 'delete':
          confirmDelete(item);
          break;
      }
    };

    // Calculate default billing period (current year)
    const currentYear = new Date().getFullYear();
    const startDate = `${currentYear}-01-01`;
    const endDate = `${currentYear}-12-31`;

    const newFixedCost = ref({
      cost_type_id: costTypeId,
      amount: '',
      billing_period_start: startDate,
      billing_period_end: endDate
    });

    // Edit modal
    const showEditModal = ref(false);
    const editingFixedCost = ref({});

    // Delete modal
    const showDeleteModal = ref(false);
    const fixedCostToDelete = ref({});

    const fetchCostType = async () => {
      try {
        const response = await costTypeService.get(costTypeId);
        costType.value = response.data;

        if (costType.value.is_consumption_based) {
          error.value = "This cost type is consumption-based and doesn't support fixed costs.";
          return;
        }
      } catch (err) {
        console.error('Error fetching cost type:', err);
        error.value = err.response?.data || 'Failed to load cost type';
      }
    };

    const fetchFixedCosts = async () => {
      loading.value = true;
      error.value = null;

      try {
        const response = await fixedCostService.getByCostType(costTypeId);
        fixedCosts.value = response.data;
      } catch (err) {
        console.error('Error fetching fixed costs:', err);
        error.value = err.response?.data || 'Failed to load fixed costs';
      } finally {
        loading.value = false;
      }
    };

    const addFixedCost = async () => {
      submitting.value = true;
      error.value = null;

      try {
        await fixedCostService.create(newFixedCost.value);
        fetchFixedCosts();
        // Reset form
        newFixedCost.value = {
          cost_type_id: costTypeId,
          amount: '',
          billing_period_start: startDate,
          billing_period_end: endDate
        };
      } catch (err) {
        console.error('Error adding fixed cost:', err);
        error.value = err.response?.data || 'Failed to add fixed cost';
      } finally {
        submitting.value = false;
      }
    };

    const editFixedCost = (cost) => {
      editingFixedCost.value = {
        id: cost.id,
        amount: cost.amount,
        billing_period_start: cost.billing_period_start,
        billing_period_end: cost.billing_period_end
      };
      showEditModal.value = true;
    };

    const updateFixedCost = async () => {
      submitting.value = true;

      try {
        await fixedCostService.update(editingFixedCost.value.id, {
          amount: editingFixedCost.value.amount,
          billing_period_start: editingFixedCost.value.billing_period_start,
          billing_period_end: editingFixedCost.value.billing_period_end
        });
        showEditModal.value = false;
        fetchFixedCosts();
      } catch (err) {
        console.error('Error updating fixed cost:', err);
        error.value = err.response?.data || 'Failed to update fixed cost';
      } finally {
        submitting.value = false;
      }
    };

    const confirmDelete = (cost) => {
      fixedCostToDelete.value = cost;
      showDeleteModal.value = true;
    };

    const deleteFixedCost = async () => {
      submitting.value = true;

      try {
        await fixedCostService.delete(fixedCostToDelete.value.id);
        showDeleteModal.value = false;
        fetchFixedCosts();
      } catch (err) {
        console.error('Error deleting fixed cost:', err);
        error.value = err.response?.data || 'Failed to delete fixed cost';
      } finally {
        submitting.value = false;
      }
    };

    const formatDate = (dateString) => {
      const date = new Date(dateString);
      return new Intl.DateTimeFormat('de-DE').format(date);
    };

    onMounted(async () => {
      await fetchCostType();
      await fetchFixedCosts();
    });

    return {
      costType,
      fixedCosts,
      loading,
      submitting,
      error,
      newFixedCost,
      showEditModal,
      editingFixedCost,
      showDeleteModal,
      fixedCostToDelete,
      tableHeaders,
      tableActions,
      handleTableAction,
      addFixedCost,
      editFixedCost,
      updateFixedCost,
      confirmDelete,
      deleteFixedCost,
      formatDate
    };
  }
};
</script>
