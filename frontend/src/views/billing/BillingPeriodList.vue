<template>
  <div>
    <PageHeader title="Billing Periods">
      <template #actions>
        <BaseButton
          @click="openCreateForm"
          variant="primary"
        >
          Add New Period
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState
      v-if="loading"
      message="Loading billing periods..."
    />

    <AlertState
      v-else-if="error"
      type="error"
      :message="error"
    />

    <EmptyState
      v-else-if="billingPeriods.length === 0"
      title="No Billing Periods Found"
      message="Create your first billing period to get started."
    >
      <template #actions>
        <BaseButton
          @click="openCreateForm"
          variant="primary"
        >
          Add New Period
        </BaseButton>
      </template>
    </EmptyState>

    <BaseTable
      v-else
      :columns="tableHeaders"
      :data="billingPeriods"
      :actions="tableActions"
      @action="handleTableAction"
    >
      <template #cell-name="{ item }">
        {{ item.name }}
      </template>
      <template #cell-property_unit="{ item }">
        {{ getPropertyUnitName(item.property_unit_id) }}
      </template>
      <template #cell-start_date="{ item }">
        {{ formatDate(item.start_date) }}
      </template>
      <template #cell-end_date="{ item }">
        {{ formatDate(item.end_date) }}
      </template>
    </BaseTable>

    <!-- Form Dialog -->
    <div v-if="showForm" class="modal-overlay">
      <BaseCard class="max-w-md mx-4">
        <h2 class="text-xl font-semibold text-gray-800 mb-6">
          {{ isEditing ? 'Edit' : 'Create' }} Billing Period
        </h2>

        <form @submit.prevent="saveBillingPeriod" class="space-y-4">
          <div>
            <label class="form-label" for="name">Name</label>
            <input
              v-model="formData.name"
              class="form-input"
              id="name"
              type="text"
              placeholder="e.g. 2024 Annual Billing"
              required
            />
          </div>

          <div>
            <label class="form-label" for="property_unit">Property Unit</label>
            <select
              v-model="formData.property_unit_id"
              class="form-input"
              id="property_unit"
              required
            >
              <option value="" disabled>Select a property unit</option>
              <option v-for="unit in propertyUnits" :key="unit.id" :value="unit.id">
                {{ unit.name }}
              </option>
            </select>
          </div>

          <div>
            <label class="form-label" for="start_date">Start Date</label>
            <input
              v-model="formData.start_date"
              class="form-input"
              id="start_date"
              type="date"
              required
            />
          </div>

          <div>
            <label class="form-label" for="end_date">End Date</label>
            <input
              v-model="formData.end_date"
              class="form-input"
              id="end_date"
              type="date"
              required
            />
          </div>

          <AlertState
            v-if="formError"
            type="error"
            :message="formError"
          />

          <div class="flex justify-end space-x-3 pt-4">
            <BaseButton
              type="button"
              variant="secondary"
              @click="closeForm"
            >
              Cancel
            </BaseButton>
            <BaseButton
              type="submit"
              variant="primary"
              :disabled="saving"
            >
              {{ saving ? 'Saving...' : 'Save' }}
            </BaseButton>
          </div>
        </form>
      </BaseCard>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteConfirmation" class="modal-overlay">
      <BaseCard class="max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          Confirm Delete
        </h3>
        <p class="text-gray-600 mb-6">
          Are you sure you want to delete the billing period "{{ periodToDelete?.name }}"?
          This will delete all statements associated with this period.
        </p>

        <div class="flex justify-end space-x-3">
          <BaseButton
            variant="secondary"
            @click="showDeleteConfirmation = false"
          >
            Cancel
          </BaseButton>
          <BaseButton
            variant="danger"
            @click="deleteBillingPeriod"
            :disabled="deleting"
          >
            {{ deleting ? 'Deleting...' : 'Delete' }}
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import billingService from '@/services/billingService';
import { propertyUnitService } from '@/services/api';
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
  name: 'BillingPeriodList',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    BaseTable,
    LoadingState,
    AlertState,
    EmptyState
  },
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
      deleting: false,
      tableHeaders: [
        { key: 'name', label: 'Name' },
        { key: 'property_unit', label: 'Property Unit' },
        { key: 'start_date', label: 'Start Date' },
        { key: 'end_date', label: 'End Date' }
      ],
      tableActions: [
        { key: 'edit', label: 'Edit', variant: 'secondary' },
        { key: 'statements', label: 'Statements', variant: 'primary' },
        { key: 'delete', label: 'Delete', variant: 'danger' }
      ]
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

    handleTableAction(action, item) {
      switch (action) {
        case 'edit':
          this.openEditForm(item);
          break;
        case 'delete':
          this.openDeleteConfirmation(item);
          break;
        case 'statements':
          this.goToStatements(item);
          break;
      }
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
