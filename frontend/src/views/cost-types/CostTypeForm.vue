<template>
  <div>
    <PageHeader :title="`${isEditing ? 'Edit' : 'Create'} Cost Type`">
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
      message="Loading..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <BaseCard v-else>
      <form @submit.prevent="saveCostType" class="space-y-6">
        <!-- Name field -->
        <div>
          <label class="form-label" for="name">
            Name <span class="text-red-500">*</span>
          </label>
          <input
            id="name"
            v-model="form.name"
            type="text"
            class="form-input"
            required
          />
        </div>

        <!-- Description field -->
        <div>
          <label class="form-label" for="description">
            Description
          </label>
          <textarea
            id="description"
            v-model="form.description"
            class="form-input h-24"
          ></textarea>
        </div>

        <!-- Cost Type field -->
        <div>
          <label class="form-label">
            Cost Type <span class="text-red-500">*</span>
          </label>
          <div class="space-y-2">
            <div class="flex items-center">
              <input
                id="consumption-based"
                v-model="form.is_consumption_based"
                type="radio"
                :value="true"
                class="form-radio mr-2"
              />
              <label for="consumption-based" class="text-gray-700">Consumption-based</label>
            </div>
            <div class="flex items-center">
              <input
                id="fixed"
                v-model="form.is_consumption_based"
                type="radio"
                :value="false"
                class="form-radio mr-2"
              />
              <label for="fixed" class="text-gray-700">Fixed</label>
            </div>
          </div>
        </div>

        <!-- Unit field (only for consumption-based) -->
        <div v-if="form.is_consumption_based">
          <label class="form-label" for="unit">
            Unit <span class="text-red-500">*</span>
          </label>
          <input
            id="unit"
            v-model="form.unit"
            type="text"
            class="form-input"
            required
            placeholder="kWh, m³, etc."
          />
        </div>

        <!-- Allocation Methods -->
        <div>
          <label class="form-label">
            Allocation Methods
          </label>

          <LoadingState
            v-if="loadingAllocationMethods"
            message="Loading allocation methods..."
            size="sm"
          />

          <div v-else class="bg-gray-50 p-4 rounded-lg space-y-2">
            <div v-for="method in allocationMethods" :key="method.id" class="flex items-center">
              <input
                :id="`method-${method.id}`"
                v-model="selectedAllocationMethods"
                type="checkbox"
                :value="method.id"
                class="form-checkbox mr-2"
              />
              <label :for="`method-${method.id}`" class="text-gray-700">
                {{ method.name }}
                <span v-if="method.description" class="text-gray-500 text-sm ml-2">
                  ({{ method.description }})
                </span>
              </label>
            </div>
          </div>
        </div>

        <!-- Submit button -->
        <div class="flex justify-end">
          <BaseButton
            type="submit"
            variant="primary"
            :disabled="submitting"
          >
            {{ submitting ? 'Saving...' : 'Save Cost Type' }}
          </BaseButton>
        </div>
      </form>
    </BaseCard>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { costTypeService } from '@/services/api';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  LoadingState,
  AlertState
} from '@/components/base';

export default {
  name: 'CostTypeForm',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    LoadingState,
    AlertState
  },
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
