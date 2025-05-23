<template>
  <div class="container mx-auto p-4">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-bold">Tariffs for {{ costType.name }}</h1>
        <p v-if="costType.description" class="text-gray-600">{{ costType.description }}</p>
      </div>
      <div class="flex space-x-2">
        <router-link
          to="/cost-types"
          class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
        >
          Back to List
        </router-link>
      </div>
    </div>

    <div v-if="loading" class="text-center py-4">
      <p class="text-gray-600">Loading tariffs...</p>
    </div>

    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <strong class="font-bold">Error:</strong>
      <span class="block sm:inline">{{ error }}</span>
    </div>

    <!-- Add Tariff Form -->
    <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-6">
      <h2 class="text-xl font-bold mb-4">Add New Tariff</h2>
      <form @submit.prevent="addTariff">
        <div class="flex flex-wrap -mx-3 mb-4">
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="price">
              Price per {{ costType.unit }} <span class="text-red-500">*</span>
            </label>
            <input
              id="price"
              v-model.number="newTariff.price_per_unit"
              type="number"
              step="0.01"
              min="0.01"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="valid-from">
              Valid From <span class="text-red-500">*</span>
            </label>
            <input
              id="valid-from"
              v-model="newTariff.valid_from"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="valid-to">
              Valid To
            </label>
            <input
              id="valid-to"
              v-model="newTariff.valid_to"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            />
          </div>
        </div>
        <div class="flex items-center justify-end">
          <button
            type="submit"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            :disabled="submitting"
          >
            {{ submitting ? 'Adding...' : 'Add Tariff' }}
          </button>
        </div>
      </form>
    </div>

    <!-- Tariffs List -->
    <div v-if="!tariffs.length" class="text-center py-6 bg-white shadow-md rounded">
      <p class="text-lg text-gray-600">No tariffs found for this cost type.</p>
    </div>

    <div v-else class="bg-white shadow overflow-hidden rounded-lg mb-6">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Price per {{ costType.unit }}
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Valid From</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Valid To</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="tariff in tariffs" :key="tariff.id">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">{{ formatCurrency(tariff.price_per_unit) }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ formatDate(tariff.valid_from) }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ tariff.valid_to ? formatDate(tariff.valid_to) : 'Ongoing' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <div class="flex space-x-2">
                <button
                  @click="editTariff(tariff)"
                  class="text-blue-600 hover:text-blue-900"
                >
                  Edit
                </button>
                <button
                  @click="confirmDelete(tariff)"
                  class="text-red-600 hover:text-red-900"
                >
                  Delete
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
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
          <div class="flex justify-end">
            <button
              type="button"
              @click="showEditModal = false"
              class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded mr-2"
            >
              Cancel
            </button>
            <button
              type="submit"
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
              :disabled="submitting"
            >
              {{ submitting ? 'Saving...' : 'Save Changes' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-xl max-w-md w-full">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Confirm Deletion</h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the tariff of {{ formatCurrency(tariffToDelete.price_per_unit) }}
          valid from {{ formatDate(tariffToDelete.valid_from) }}? This action cannot be undone.
        </p>
        <div class="flex justify-end">
          <button
            @click="showDeleteModal = false"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded mr-2"
          >
            Cancel
          </button>
          <button
            @click="deleteTariff"
            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
            :disabled="submitting"
          >
            {{ submitting ? 'Deleting...' : 'Delete' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { costTypeService, tariffService } from '@/services/api';

export default {
  name: 'TariffList',
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
