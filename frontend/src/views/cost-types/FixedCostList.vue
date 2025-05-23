<template>
  <div class="container mx-auto p-4">
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-bold">Fixed Costs for {{ costType.name }}</h1>
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
      <p class="text-gray-600">Loading fixed costs...</p>
    </div>

    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <strong class="font-bold">Error:</strong>
      <span class="block sm:inline">{{ error }}</span>
    </div>

    <!-- Add Fixed Cost Form -->
    <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-6">
      <h2 class="text-xl font-bold mb-4">Add New Fixed Cost</h2>
      <form @submit.prevent="addFixedCost">
        <div class="flex flex-wrap -mx-3 mb-4">
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="amount">
              Amount <span class="text-red-500">*</span>
            </label>
            <input
              id="amount"
              v-model.number="newFixedCost.amount"
              type="number"
              step="0.01"
              min="0.01"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="billing-period-start">
              Billing Period Start <span class="text-red-500">*</span>
            </label>
            <input
              id="billing-period-start"
              v-model="newFixedCost.billing_period_start"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="billing-period-end">
              Billing Period End <span class="text-red-500">*</span>
            </label>
            <input
              id="billing-period-end"
              v-model="newFixedCost.billing_period_end"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
        </div>
        <div class="flex items-center justify-end">
          <button
            type="submit"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            :disabled="submitting"
          >
            {{ submitting ? 'Adding...' : 'Add Fixed Cost' }}
          </button>
        </div>
      </form>
    </div>

    <!-- Fixed Costs List -->
    <div v-if="!fixedCosts.length" class="text-center py-6 bg-white shadow-md rounded">
      <p class="text-lg text-gray-600">No fixed costs found for this cost type.</p>
    </div>

    <div v-else class="bg-white shadow overflow-hidden rounded-lg mb-6">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Amount</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Billing Period</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="cost in fixedCosts" :key="cost.id">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">{{ formatCurrency(cost.amount) }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">
                {{ formatDate(cost.billing_period_start) }} - {{ formatDate(cost.billing_period_end) }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <div class="flex space-x-2">
                <button
                  @click="editFixedCost(cost)"
                  class="text-blue-600 hover:text-blue-900"
                >
                  Edit
                </button>
                <button
                  @click="confirmDelete(cost)"
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

    <!-- Edit Fixed Cost Modal -->
    <div v-if="showEditModal" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-xl max-w-md w-full">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Edit Fixed Cost</h3>
        <form @submit.prevent="updateFixedCost">
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-amount">
              Amount <span class="text-red-500">*</span>
            </label>
            <input
              id="edit-amount"
              v-model.number="editingFixedCost.amount"
              type="number"
              step="0.01"
              min="0.01"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-billing-period-start">
              Billing Period Start <span class="text-red-500">*</span>
            </label>
            <input
              id="edit-billing-period-start"
              v-model="editingFixedCost.billing_period_start"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
            />
          </div>
          <div class="mb-6">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="edit-billing-period-end">
              Billing Period End <span class="text-red-500">*</span>
            </label>
            <input
              id="edit-billing-period-end"
              v-model="editingFixedCost.billing_period_end"
              type="date"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              required
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
          Are you sure you want to delete the fixed cost of {{ formatCurrency(fixedCostToDelete.amount) }}
          for period {{ formatDate(fixedCostToDelete.billing_period_start) }} - {{ formatDate(fixedCostToDelete.billing_period_end) }}?
          This action cannot be undone.
        </p>
        <div class="flex justify-end">
          <button
            @click="showDeleteModal = false"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded mr-2"
          >
            Cancel
          </button>
          <button
            @click="deleteFixedCost"
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
import { costTypeService, fixedCostService } from '@/services/api';

export default {
  name: 'FixedCostList',
  props: {
    id: {
      type: String,
      required: true
    }
  },  setup(props) {
    const costTypeId = parseInt(props.id);

    const costType = ref({});
    const fixedCosts = ref([]);
    const loading = ref(true);
    const submitting = ref(false);
    const error = ref(null);

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
      addFixedCost,
      editFixedCost,
      updateFixedCost,
      confirmDelete,
      deleteFixedCost,
      formatCurrency,
      formatDate
    };
  }
};
</script>
