<template>
  <div>
    <PageHeader :title="`Tariffs for ${costType.name}`">
      <template #subtitle>
        <p v-if="costType.description" class="text-gray-600">{{ costType.description }}</p>
      </template>
      <template #actions>
        <BaseButton
          variant="secondary"
          @click="$router.push('/cost-types')"
        >
          Back to List
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState v-if="loading" message="Loading tariffs..." />

    <AlertState v-else-if="error" type="error" :message="error" />

    <div v-else>
      <!-- Add Tariff Form -->
      <BaseCard class="mb-6">
        <h2 class="text-xl font-bold mb-4">Add New Tariff</h2>
        <form @submit.prevent="addTariff">
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
            <div>
              <label class="form-label" for="price">
                Price per {{ costType.unit }} <span class="text-red-500">*</span>
              </label>
              <input
                id="price"
                v-model.number="newTariff.price_per_unit"
                type="number"
                step="0.01"
                min="0.01"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="valid-from">
                Valid From <span class="text-red-500">*</span>
              </label>
              <input
                id="valid-from"
                v-model="newTariff.valid_from"
                type="date"
                class="form-input"
                required
              />
            </div>
            <div>
              <label class="form-label" for="valid-to">
                Valid To
              </label>
              <input
                id="valid-to"
                v-model="newTariff.valid_to"
                type="date"
                class="form-input"
              />
            </div>
          </div>
          <div class="flex justify-end">
            <BaseButton
              type="submit"
              variant="primary"
              :disabled="submitting"
            >
              {{ submitting ? 'Adding...' : 'Add Tariff' }}
            </BaseButton>
          </div>
        </form>
      </BaseCard>

      <!-- Tariffs List -->
      <EmptyState
        v-if="!tariffs.length"
        title="No tariffs found"
        description="No tariffs found for this cost type."
      />

      <BaseTable
        v-else
        :columns="tableHeaders"
        :items="tariffs"
        :actions="tableActions"
        @action="handleTableAction"
      >
        <template #cell-price_per_unit="{ item }">
          <span class="font-medium">{{ formatCurrency(item.price_per_unit) }}</span>
        </template>

        <template #cell-valid_from="{ item }">
          {{ formatDate(item.valid_from) }}
        </template>

        <template #cell-valid_to="{ item }">
          {{ item.valid_to ? formatDate(item.valid_to) : 'Ongoing' }}
        </template>
      </BaseTable>
    </div>

    <!-- Edit Tariff Modal -->
    <div v-if="showEditModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-xl max-w-md w-full">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Edit Tariff</h3>
        <form @submit.prevent="updateTariff">
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-price">
              Price per {{ costType.unit }} <span class="text-red-500">*</span>
            </label>
            <input
              id="edit-price"
              v-model.number="editingTariff.price_per_unit"
              type="number"
              step="0.01"
              min="0.01"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-valid-from">
              Valid From <span class="text-red-500">*</span>
            </label>
            <input
              id="edit-valid-from"
              v-model="editingTariff.valid_from"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="mb-6">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-valid-to">
              Valid To
            </label>
            <input
              id="edit-valid-to"
              v-model="editingTariff.valid_to"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            />
          </div>
          <div class="flex justify-end space-x-3">
            <BaseButton
              variant="secondary"
              @click="showEditModal = false"
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
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="modal-overlay">
      <BaseCard class="modal-content max-w-md">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Confirm Deletion</h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the tariff of {{ formatCurrency(tariffToDelete.price_per_unit) }}
          valid from {{ formatDate(tariffToDelete.valid_from) }}? This action cannot be undone.
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
            @click="deleteTariff"
            :disabled="submitting"
          >
            {{ submitting ? 'Deleting...' : 'Delete' }}
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { costTypeService, tariffService } from '@/services/api';
import PageHeader from '@/components/base/PageHeader.vue';
import BaseCard from '@/components/base/BaseCard.vue';
import BaseButton from '@/components/base/BaseButton.vue';
import BaseTable from '@/components/base/BaseTable.vue';
import LoadingState from '@/components/base/LoadingState.vue';
import AlertState from '@/components/base/AlertState.vue';
import EmptyState from '@/components/base/EmptyState.vue';

export default {
  name: 'TariffList',
  components: {
    PageHeader,
    BaseCard,
    BaseButton,
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
  },  setup(props) {
    const costTypeId = parseInt(props.id);

    const costType = ref({});
    const tariffs = ref([]);
    const loading = ref(true);
    const submitting = ref(false);
    const error = ref(null);

    const newTariff = ref({
      cost_type_id: costTypeId,
      price_per_unit: '',
      valid_from: new Date().toISOString().split('T')[0], // Today's date
      valid_to: null
    });

    // Edit modal
    const showEditModal = ref(false);
    const editingTariff = ref({});

    // Delete modal
    const showDeleteModal = ref(false);
    const tariffToDelete = ref({});

    const tableHeaders = [
      { key: 'price_per_unit', label: `Price per ${costType.value.unit || 'Unit'}` },
      { key: 'valid_from', label: 'Valid From' },
      { key: 'valid_to', label: 'Valid To' }
    ];

    const tableActions = [
      { key: 'edit', label: 'Edit', variant: 'primary' },
      { key: 'delete', label: 'Delete', variant: 'danger' }
    ];

    const handleTableAction = (action, item) => {
      if (action === 'edit') {
        editTariff(item);
      } else if (action === 'delete') {
        confirmDelete(item);
      }
    };

    const fetchCostType = async () => {
      try {
        const response = await costTypeService.get(costTypeId);
        costType.value = response.data;

        if (!costType.value.is_consumption_based) {
          error.value = "This cost type is not consumption-based and doesn't support tariffs.";
          return;
        }
      } catch (err) {
        console.error('Error fetching cost type:', err);
        error.value = err.response?.data || 'Failed to load cost type';
      }
    };

    const fetchTariffs = async () => {
      loading.value = true;
      error.value = null;

      try {
        const response = await tariffService.getByCostType(costTypeId);
        tariffs.value = response.data;
      } catch (err) {
        console.error('Error fetching tariffs:', err);
        error.value = err.response?.data || 'Failed to load tariffs';
      } finally {
        loading.value = false;
      }
    };

    const addTariff = async () => {
      submitting.value = true;
      error.value = null;

      try {
        await tariffService.create(newTariff.value);
        fetchTariffs();
        // Reset form
        newTariff.value = {
          cost_type_id: costTypeId,
          price_per_unit: '',
          valid_from: new Date().toISOString().split('T')[0],
          valid_to: null
        };
      } catch (err) {
        console.error('Error adding tariff:', err);
        error.value = err.response?.data || 'Failed to add tariff';
      } finally {
        submitting.value = false;
      }
    };

    const editTariff = (tariff) => {
      editingTariff.value = {
        id: tariff.id,
        price_per_unit: tariff.price_per_unit,
        valid_from: tariff.valid_from,
        valid_to: tariff.valid_to
      };
      showEditModal.value = true;
    };

    const updateTariff = async () => {
      submitting.value = true;

      try {
        await tariffService.update(editingTariff.value.id, {
          price_per_unit: editingTariff.value.price_per_unit,
          valid_from: editingTariff.value.valid_from,
          valid_to: editingTariff.value.valid_to || null
        });
        showEditModal.value = false;
        fetchTariffs();
      } catch (err) {
        console.error('Error updating tariff:', err);
        error.value = err.response?.data || 'Failed to update tariff';
      } finally {
        submitting.value = false;
      }
    };

    const confirmDelete = (tariff) => {
      tariffToDelete.value = tariff;
      showDeleteModal.value = true;
    };

    const deleteTariff = async () => {
      submitting.value = true;

      try {
        await tariffService.delete(tariffToDelete.value.id);
        showDeleteModal.value = false;
        fetchTariffs();
      } catch (err) {
        console.error('Error deleting tariff:', err);
        error.value = err.response?.data || 'Failed to delete tariff';
      } finally {
        submitting.value = false;
      }
    };

    const formatCurrency = (amount) => {
      return new Intl.NumberFormat('de-DE', {
        style: 'currency',
        currency: 'EUR'
      }).format(amount);
    };

    const formatDate = (dateString) => {
      const date = new Date(dateString);
      return new Intl.DateTimeFormat('de-DE').format(date);
    };

    onMounted(async () => {
      await fetchCostType();
      await fetchTariffs();
    });

    return {
      costType,
      tariffs,
      loading,
      submitting,
      error,
      newTariff,
      showEditModal,
      editingTariff,
      showDeleteModal,
      tariffToDelete,
      tableHeaders,
      tableActions,
      handleTableAction,
      addTariff,
      editTariff,
      updateTariff,
      confirmDelete,
      deleteTariff,
      formatCurrency,
      formatDate
    };
  }
};
</script>

<style scoped>
.modal-overlay {
  @apply fixed inset-0 flex items-center justify-center z-50;
  background-color: rgba(0, 0, 0, 0.5);
}

.modal-content {
  @apply bg-white p-6 rounded-lg shadow-lg z-10;
}
</style>
