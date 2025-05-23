<template>
  <div class="container mx-auto p-4">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">Cost Types</h1>
      <router-link
        to="/cost-types/create"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
      >
        + Add Cost Type
      </router-link>
    </div>

    <div v-if="loading" class="text-center py-4">
      <p class="text-gray-600">Loading cost types...</p>
    </div>

    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <strong class="font-bold">Error:</strong>
      <span class="block sm:inline">{{ error }}</span>
    </div>

    <div v-else-if="!costTypes.length" class="text-center py-6">
      <p class="text-lg text-gray-600">No cost types found. Create your first cost type!</p>
    </div>

    <div v-else class="bg-white shadow overflow-hidden rounded-lg mb-6">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Type</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Unit</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Allocation Methods</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="costType in costTypes" :key="costType.id">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">{{ costType.name }}</div>
              <div v-if="costType.description" class="text-sm text-gray-500">
                {{ costType.description }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                :class="[
                  'px-2 inline-flex text-xs leading-5 font-semibold rounded-full',
                  costType.is_consumption_based ? 'bg-green-100 text-green-800' : 'bg-blue-100 text-blue-800'
                ]"
              >
                {{ costType.is_consumption_based ? 'Consumption' : 'Fixed' }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ costType.unit || '-' }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="method in costType.allocation_methods"
                  :key="method.id"
                  class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800"
                >
                  {{ method.name }}
                </span>
                <span v-if="!costType.allocation_methods.length" class="text-sm text-gray-500">
                  None
                </span>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <div class="flex space-x-2">
                <router-link
                  :to="{ name: 'CostTypeEdit', params: { id: costType.id } }"
                  class="text-blue-600 hover:text-blue-900 mr-2"
                >
                  Edit
                </router-link>
                <template v-if="costType.is_consumption_based">
                  <router-link
                    :to="{ name: 'CostTypeTariffs', params: { id: costType.id } }"
                    class="text-purple-600 hover:text-purple-900 mr-2"
                  >
                    Tariffs
                  </router-link>
                </template>
                <template v-else>
                  <router-link
                    :to="{ name: 'CostTypeFixedCosts', params: { id: costType.id } }"
                    class="text-green-600 hover:text-green-900 mr-2"
                  >
                    Fixed Costs
                  </router-link>
                </template>
                <button
                  @click="confirmDelete(costType)"
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

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-xl max-w-md w-full">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Confirm Deletion</h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the cost type "{{ costTypeToDelete.name }}"? This action cannot be undone.
        </p>
        <div class="flex justify-end">
          <button
            @click="showDeleteModal = false"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded mr-2"
          >
            Cancel
          </button>
          <button
            @click="deleteCostType"
            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { costTypeService } from '@/services/api';

export default {
  name: 'CostTypeList',
  setup() {
    const costTypes = ref([]);
    const loading = ref(true);
    const error = ref(null);
    const showDeleteModal = ref(false);
    const costTypeToDelete = ref({});

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
      confirmDelete,
      deleteCostType
    };
  }
};
</script>
