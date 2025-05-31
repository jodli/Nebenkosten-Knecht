<template>
  <div>
    <PageHeader title="Meter Readings">
      <template #actions>
        <BaseButton
          variant="primary"
          @click="$router.push('/meter-readings/create')"
        >
          Add Reading
        </BaseButton>
      </template>
    </PageHeader>

    <LoadingState v-if="loading" message="Loading meter readings..." />

    <AlertState v-else-if="error" type="error" :message="error" />

    <EmptyState
      v-else-if="meterReadings.length === 0"
      title="No meter readings found"
      description="Create your first meter reading to get started."
    >
      <template #actions>
        <BaseButton
          variant="primary"
          @click="$router.push('/meter-readings/create')"
        >
          Add Reading
        </BaseButton>
      </template>
    </EmptyState>

    <div v-else>
      <BaseCard class="mb-6">
        <div class="flex items-center space-x-2">
          <label for="meterFilter" class="text-gray-700 font-medium">Filter by Meter:</label>
          <select
            id="meterFilter"
            v-model="selectedMeterId"
            class="form-input"
            @change="filterByMeter"
          >
            <option value="">All Meters</option>
            <option v-for="meter in meters" :key="meter.id" :value="meter.id">
              {{ meter.name }} ({{ meter.meter_type }})
            </option>
          </select>
        </div>
      </BaseCard>

      <BaseTable
        :columns="tableHeaders"
        :items="meterReadings"
        :actions="tableActions"
        @action="handleTableAction"
      >
        <template #cell-meter="{ item }">
          <div v-if="getMeterById(item.meter_id)">
            {{ getMeterById(item.meter_id).name }}
            <span class="text-xs text-gray-500">
              ({{ getMeterById(item.meter_id).meter_type }})
            </span>
          </div>
          <div v-else class="text-gray-400">
            Unknown Meter
          </div>
        </template>

        <template #reading_date="{ item }">
          {{ formatDate(item.reading_date) }}
        </template>

        <template #value="{ item }">
          {{ item.value.toFixed(2) }}
          <span class="text-xs text-gray-500" v-if="getMeterById(item.meter_id)">
            {{ getMeterById(item.meter_id).unit }}
          </span>
        </template>

        <template #notes="{ item }">
          {{ item.notes || '-' }}
        </template>
      </BaseTable>

      <div class="mt-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-3">
          View by Meter
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div v-for="meter in meters" :key="meter.id" class="card p-4 hover:bg-gray-50 cursor-pointer"
              @click="goToMeterReadings(meter.id)">
            <h3 class="font-semibold text-blue-600">{{ meter.name }}</h3>
            <div class="text-sm text-gray-600">
              {{ meter.meter_type }} ({{ meter.unit }})
            </div>
            <div class="mt-2 text-xs">
              <span class="text-green-600">
                View readings and consumption
              </span>
            </div>
          </div>
        </div>
      </div>
      <!-- Consumption Summary Section -->
      <BaseCard class="mt-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-3">
          Consumption Summary
        </h2>
        <p class="text-gray-600 mb-4">
          View consumption analysis for each meter to track usage patterns.
        </p>
        <div class="space-y-2">
          <BaseButton
            v-for="meter in meters"
            :key="meter.id"
            variant="outline"
            size="sm"
            @click="goToMeterReadings(meter.id)"
            class="mr-2 mb-2"
          >
            {{ meter.name }} ({{ meter.meter_type }})
          </BaseButton>
        </div>
      </BaseCard>
    </div>

    <!-- Confirmation Dialog -->
    <div v-if="showConfirmation" class="modal-overlay">
      <BaseCard class="modal-content max-w-md mx-auto">
        <h3 class="text-lg font-semibold mb-4">
          Confirm Deletion
        </h3>
        <p>
          Are you sure you want to delete the meter reading from
          <span class="font-semibold">{{ selectedReading ? formatDate(selectedReading.reading_date) : '' }}</span>?
          This action cannot be undone.
        </p>
        <div class="mt-6 flex justify-end space-x-3">
          <BaseButton
            variant="secondary"
            @click="showConfirmation = false"
          >
            Cancel
          </BaseButton>
          <BaseButton
            variant="danger"
            @click="deleteReading"
          >
            Delete
          </BaseButton>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import { meterReadingService, meterService } from '@/services/api';
import PageHeader from '@/components/base/PageHeader.vue';
import BaseCard from '@/components/base/BaseCard.vue';
import BaseButton from '@/components/base/BaseButton.vue';
import BaseTable from '@/components/base/BaseTable.vue';
import LoadingState from '@/components/base/LoadingState.vue';
import AlertState from '@/components/base/AlertState.vue';
import EmptyState from '@/components/base/EmptyState.vue';

export default {
    name: 'MeterReadingList',
    components: {
        PageHeader,
        BaseCard,
        BaseButton,
        BaseTable,
        LoadingState,
        AlertState,
        EmptyState
    },

    data() {
        return {
            meterReadings: [],
            meters: [],
            loading: true,
            error: null,
            showConfirmation: false,
            selectedReading: null,
            selectedMeterId: '',
            tableHeaders: [
                { key: 'meter', label: 'Meter' },
                { key: 'reading_date', label: 'Reading Date' },
                { key: 'value', label: 'Reading Value', align: 'right' },
                { key: 'notes', label: 'Notes' }
            ],
            tableActions: [
                { key: 'edit', label: 'Edit', variant: 'primary' },
                { key: 'delete', label: 'Delete', variant: 'danger' }
            ]
        };
    },

    methods: {
        async fetchMeterReadings() {
            this.loading = true;
            try {
                const response = await meterReadingService.getAll();
                this.meterReadings = response.data;
                this.error = null;
            } catch (err) {
                console.error('Error fetching meter readings:', err);
                this.error = 'Failed to load meter readings. Please try again.';
            } finally {
                this.loading = false;
            }
        },

        async fetchMeters() {
            try {
                const response = await meterService.getAll();
                this.meters = response.data;
            } catch (err) {
                console.error('Error fetching meters:', err);
            }
        },

        getMeterById(meterId) {
            return this.meters.find(meter => meter.id === meterId);
        },

        formatDate(dateString) {
            return new Date(dateString).toLocaleDateString();
        },

        handleTableAction(action, item) {
            if (action === 'edit') {
                this.$router.push(`/meter-readings/${item.id}`);
            } else if (action === 'delete') {
                this.confirmDelete(item);
            }
        },

        confirmDelete(reading) {
            this.selectedReading = reading;
            this.showConfirmation = true;
        },

        async deleteReading() {
            try {
                await meterReadingService.delete(this.selectedReading.id);
                this.meterReadings = this.meterReadings.filter(r => r.id !== this.selectedReading.id);
                this.showConfirmation = false;
                this.selectedReading = null;
            } catch (err) {
                console.error('Error deleting meter reading:', err);
                this.error = 'Failed to delete meter reading. Please try again.';
            }
        },

        async filterByMeter() {
            this.loading = true;
            try {
                if (this.selectedMeterId) {
                    const response = await meterReadingService.getByMeter(this.selectedMeterId);
                    this.meterReadings = response.data;
                } else {
                    await this.fetchMeterReadings();
                }
                this.error = null;
            } catch (err) {
                console.error('Error filtering meter readings:', err);
                this.error = 'Failed to filter meter readings. Please try again.';
            } finally {
                this.loading = false;
            }
        },

        goToMeterReadings(meterId) {
            this.$router.push(`/meter-readings/meter/${meterId}`);
        }
    },

    async created() {
        await Promise.all([this.fetchMeterReadings(), this.fetchMeters()]);
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
