<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-2xl font-bold">Billing Periods</h1>
      <button
        @click="openCreateForm"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Add New Period
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-8">
      <div class="spinner"></div>
      <p class="mt-2">Loading billing periods...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <p>{{ error }}</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="billingPeriods.length === 0" class="text-center py-8">
      <p class="text-gray-500">No billing periods found. Create your first billing period to get started.</p>
    </div>

    <!-- Billing Periods List -->
    <div v-else class="overflow-x-auto">
      <table class="min-w-full bg-white">
        <thead>
          <tr>
            <th class="py-2 px-4 border-b border-gray-200 bg-gray-50 text-left">Name</th>
            <th class="py-2 px-4 border-b border-gray-200 bg-gray-50 text-left">Property Unit</th>
            <th class="py-2 px-4 border-b border-gray-200 bg-gray-50 text-left">Start Date</th>
            <th class="py-2 px-4 border-b border-gray-200 bg-gray-50 text-left">End Date</th>
            <th class="py-2 px-4 border-b border-gray-200 bg-gray-50 text-left">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="period in billingPeriods" :key="period.id" class="hover:bg-gray-100">
            <td class="py-2 px-4 border-b border-gray-200">{{ period.name }}</td>
            <td class="py-2 px-4 border-b border-gray-200">
              {{ getPropertyUnitName(period.property_unit_id) }}
            </td>
            <td class="py-2 px-4 border-b border-gray-200">{{ formatDate(period.start_date) }}</td>
            <td class="py-2 px-4 border-b border-gray-200">{{ formatDate(period.end_date) }}</td>
            <td class="py-2 px-4 border-b border-gray-200">
              <div class="flex space-x-2">
                <button
                  @click="openEditForm(period)"
                  class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm"
                >
                  Edit
                </button>
                <button
                  @click="openDeleteConfirmation(period)"
                  class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-sm"
                >
                  Delete
                </button>
                <button
                  @click="goToStatements(period)"
                  class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm"
                >
                  Statements
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Form Dialog -->
    <div v-if="showForm" class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-lg max-w-md w-full">
        <h2 class="text-xl font-bold mb-4">{{ isEditing ? 'Edit' : 'Create' }} Billing Period</h2>

        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="name">Name</label>
          <input
            v-model="formData.name"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="name"
            type="text"
            placeholder="e.g. 2024 Annual Billing"
          />
        </div>

        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="property_unit">Property Unit</label>
          <select
            v-model="formData.property_unit_id"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="property_unit"
          >
            <option value="" disabled>Select a property unit</option>
            <option v-for="unit in propertyUnits" :key="unit.id" :value="unit.id">
              {{ unit.name }}
            </option>
          </select>
        </div>

        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="start_date">Start Date</label>
          <input
            v-model="formData.start_date"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="start_date"
            type="date"
          />
        </div>

        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="end_date">End Date</label>
          <input
            v-model="formData.end_date"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="end_date"
            type="date"
          />
        </div>

        <!-- Validation Error -->
        <div v-if="formError" class="mb-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <p>{{ formError }}</p>
        </div>

        <div class="flex justify-end space-x-2">
          <button
            @click="closeForm"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded"
          >
            Cancel
          </button>
          <button
            @click="saveBillingPeriod"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            :disabled="saving"
          >
            {{ saving ? 'Saving...' : 'Save' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteConfirmation" class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      <div class="bg-white p-6 rounded-lg shadow-lg max-w-md w-full">
        <h2 class="text-xl font-bold mb-4">Confirm Delete</h2>
        <p class="mb-4">
          Are you sure you want to delete the billing period "{{ periodToDelete.name }}"?
          This will delete all statements associated with this period.
        </p>

        <div class="flex justify-end space-x-2">
          <button
            @click="showDeleteConfirmation = false"
            class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded"
          >
            Cancel
          </button>
          <button
            @click="deleteBillingPeriod"
            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
            :disabled="deleting"
          >
            {{ deleting ? 'Deleting...' : 'Delete' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import billingService from '@/services/billingService';
import { propertyUnitService } from '@/services/api';

export default {
  name: 'BillingPeriodList',
  data() {
    return {
      billingPeriods: [],
      propertyUnits: [],
      loading: true,
      error: null,
      showForm: false,
      isEditing: false,
      formData: {
        name: '',
        property_unit_id: '',
        start_date: '',
        end_date: ''
      },
      formError: null,
      saving: false,
      showDeleteConfirmation: false,
      periodToDelete: null,
      deleting: false
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      this.loading = true;
      this.error = null;

      try {
        const [billingResponse, unitResponse] = await Promise.all([
          billingService.getAllBillingPeriods(),
          propertyUnitService.getAll()
        ]);

        this.billingPeriods = billingResponse.data;
        this.propertyUnits = unitResponse.data;
      } catch (error) {
        console.error('Error fetching data:', error);
        this.error = 'Failed to load data. Please try again later.';
      } finally {
        this.loading = false;
      }
    },

    getPropertyUnitName(id) {
      const unit = this.propertyUnits.find(u => u.id === id);
      return unit ? unit.name : 'Unknown';
    },

    formatDate(dateString) {
      const date = new Date(dateString);
      return date.toLocaleDateString();
    },

    openCreateForm() {
      this.isEditing = false;
      this.formData = {
        name: '',
        property_unit_id: '',
        start_date: '',
        end_date: ''
      };
      this.formError = null;
      this.showForm = true;
    },

    openEditForm(period) {
      this.isEditing = true;
      this.formData = {
        name: period.name,
        property_unit_id: period.property_unit_id,
        start_date: period.start_date,
        end_date: period.end_date
      };
      this.periodToEdit = period;
      this.formError = null;
      this.showForm = true;
    },

    closeForm() {
      this.showForm = false;
    },

    async saveBillingPeriod() {
      // Form validation
      if (!this.formData.name) {
        this.formError = 'Name is required.';
        return;
      }

      if (!this.formData.property_unit_id) {
        this.formError = 'Property unit is required.';
        return;
      }

      if (!this.formData.start_date) {
        this.formError = 'Start date is required.';
        return;
      }

      if (!this.formData.end_date) {
        this.formError = 'End date is required.';
        return;
      }

      if (new Date(this.formData.start_date) > new Date(this.formData.end_date)) {
        this.formError = 'Start date must be before end date.';
        return;
      }

      this.saving = true;

      try {
        if (this.isEditing) {
          await billingService.updateBillingPeriod(this.periodToEdit.id, this.formData);
        } else {
          await billingService.createBillingPeriod(this.formData);
        }

        await this.fetchData();
        this.showForm = false;
      } catch (error) {
        console.error('Error saving billing period:', error);
        if (error.response && error.response.data) {
          this.formError = error.response.data;
        } else {
          this.formError = 'Failed to save billing period. Please try again.';
        }
      } finally {
        this.saving = false;
      }
    },

    openDeleteConfirmation(period) {
      this.periodToDelete = period;
      this.showDeleteConfirmation = true;
    },

    async deleteBillingPeriod() {
      this.deleting = true;

      try {
        await billingService.deleteBillingPeriod(this.periodToDelete.id);
        await this.fetchData();
        this.showDeleteConfirmation = false;
      } catch (error) {
        console.error('Error deleting billing period:', error);
        this.error = 'Failed to delete billing period. Please try again later.';
      } finally {
        this.deleting = false;
      }
    },

    goToStatements(period) {
      this.$router.push({
        name: 'BillingStatementList',
        params: { periodId: period.id }
      });
    }
  }
};
</script>

<style scoped>
.spinner {
  border: 4px solid rgba(0, 0, 0, 0.1);
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border-left-color: #3490dc;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
