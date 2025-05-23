<template>
  <div class="container mx-auto p-4">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">{{ isEditing ? 'Edit' : 'Create' }} Cost Type</h1>
      <router-link
        to="/cost-types"
        class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
      >
        Back to List
      </router-link>
    </div>

    <div v-if="loading" class="text-center py-4">
      <p class="text-gray-600">Loading...</p>
    </div>

    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <strong class="font-bold">Error:</strong>
      <span class="block sm:inline">{{ error }}</span>
    </div>

    <form v-else @submit.prevent="saveCostType" class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
      <!-- Name field -->
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="name">
          Name <span class="text-red-500">*</span>
        </label>
        <input
          id="name"
          v-model="form.name"
          type="text"
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          required
        />
      </div>

      <!-- Description field -->
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
          Description
        </label>
        <textarea
          id="description"
          v-model="form.description"
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline h-24"
        ></textarea>
      </div>

      <!-- Cost Type field -->
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2">
          Cost Type <span class="text-red-500">*</span>
        </label>
        <div class="flex items-center mb-2">
          <input
            id="consumption-based"
            v-model="form.is_consumption_based"
            type="radio"
            :value="true"
            class="mr-2"
          />
          <label for="consumption-based" class="text-gray-700">Consumption-based</label>
        </div>
        <div class="flex items-center">
          <input
            id="fixed"
            v-model="form.is_consumption_based"
            type="radio"
            :value="false"
            class="mr-2"
          />
          <label for="fixed" class="text-gray-700">Fixed</label>
        </div>
      </div>

      <!-- Unit field (only for consumption-based) -->
      <div v-if="form.is_consumption_based" class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="unit">
          Unit <span class="text-red-500">*</span>
        </label>
        <input
          id="unit"
          v-model="form.unit"
          type="text"
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          required
          placeholder="kWh, m³, etc."
        />
      </div>

      <!-- Allocation Methods -->
      <div class="mb-6">
        <label class="block text-gray-700 text-sm font-bold mb-2">
          Allocation Methods
        </label>
        <div v-if="loadingAllocationMethods" class="text-gray-600">Loading allocation methods...</div>
        <div v-else class="bg-gray-100 p-4 rounded">
          <div v-for="method in allocationMethods" :key="method.id" class="flex items-center mb-2">
            <input
              :id="`method-${method.id}`"
              v-model="selectedAllocationMethods"
              type="checkbox"
              :value="method.id"
              class="mr-2"
            />
            <label :for="`method-${method.id}`" class="text-gray-700">
              {{ method.name }}
              <span v-if="method.description" class="text-gray-500 text-xs ml-2">
                ({{ method.description }})
              </span>
            </label>
          </div>
        </div>
      </div>

      <!-- Submit button -->
      <div class="flex items-center justify-between">
        <button
          type="submit"
          class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
          :disabled="submitting"
        >
          {{ submitting ? 'Saving...' : 'Save Cost Type' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { costTypeService } from '@/services/api';

export default {
  name: 'CostTypeForm',
  props: {
    id: {
      type: String,
      default: null
    }
  },
  setup(props) {
    const router = useRouter();
    const isEditing = computed(() => !!props.id);

    const form = ref({
      name: '',
      description: '',
      is_consumption_based: false,
      unit: null
    });

    const loading = ref(false);
    const submitting = ref(false);
    const error = ref(null);

    const allocationMethods = ref([]);
    const loadingAllocationMethods = ref(false);
    const selectedAllocationMethods = ref([]);

    const fetchAllocationMethods = async () => {
      loadingAllocationMethods.value = true;
      try {
        const response = await costTypeService.getAllocationMethods();
        allocationMethods.value = response.data;
      } catch (err) {
        console.error('Error fetching allocation methods:', err);
        error.value = 'Failed to load allocation methods';
      } finally {
        loadingAllocationMethods.value = false;
      }
    };

    const fetchCostType = async () => {
      if (!props.id) return;

      loading.value = true;
      error.value = null;

      try {
        const response = await costTypeService.get(props.id);
        const costType = response.data;

        form.value = {
          name: costType.name,
          description: costType.description || '',
          is_consumption_based: costType.is_consumption_based,
          unit: costType.unit
        };

        selectedAllocationMethods.value = costType.allocation_methods.map(method => method.id);
      } catch (err) {
        console.error('Error fetching cost type:', err);
        error.value = err.response?.data || 'Failed to load cost type';
      } finally {
        loading.value = false;
      }
    };

    const saveCostType = async () => {
      // Validation
      if (!form.value.name.trim()) {
        error.value = 'Name is required';
        return;
      }

      if (form.value.is_consumption_based && !form.value.unit) {
        error.value = 'Unit is required for consumption-based cost types';
        return;
      }

      submitting.value = true;
      error.value = null;

      try {
        let costTypeId;

        if (isEditing.value) {
          // Update existing cost type
          const response = await costTypeService.update(props.id, {
            name: form.value.name,
            description: form.value.description || null,
            is_consumption_based: form.value.is_consumption_based,
            unit: form.value.unit
          });
          costTypeId = response.data.id;
        } else {
          // Create new cost type
          const response = await costTypeService.create({
            name: form.value.name,
            description: form.value.description || null,
            is_consumption_based: form.value.is_consumption_based,
            unit: form.value.unit
          });
          costTypeId = response.data.id;
        }

        // Handle allocation methods
        // Get current allocation methods for the cost type
        const currentResponse = await costTypeService.get(costTypeId);
        const currentMethods = currentResponse.data.allocation_methods.map(method => method.id);

        // Methods to add
        const methodsToAdd = selectedAllocationMethods.value.filter(id => !currentMethods.includes(id));

        // Methods to remove
        const methodsToRemove = currentMethods.filter(id => !selectedAllocationMethods.value.includes(id));

        // Add new methods
        for (const methodId of methodsToAdd) {
          await costTypeService.assignAllocationMethod(costTypeId, methodId);
        }

        // Remove old methods
        for (const methodId of methodsToRemove) {
          await costTypeService.removeAllocationMethod(costTypeId, methodId);
        }

        // Navigate back to the list
        router.push('/cost-types');
      } catch (err) {
        console.error('Error saving cost type:', err);
        error.value = err.response?.data || 'An error occurred while saving the cost type';
      } finally {
        submitting.value = false;
      }
    };

    onMounted(() => {
      fetchAllocationMethods();
      if (isEditing.value) {
        fetchCostType();
      }
    });

    return {
      form,
      loading,
      submitting,
      error,
      isEditing,
      saveCostType,
      allocationMethods,
      loadingAllocationMethods,
      selectedAllocationMethods
    };
  }
};
</script>
