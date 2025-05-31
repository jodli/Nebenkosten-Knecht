<template>
    <div>
        <PageHeader :title="`Readings for ${meter ? meter.name : 'Loading...'}`">
            <template #actions>
                <BaseButton
                    @click="$router.push({ name: 'MeterConsumption', params: { meterId } })"
                    variant="secondary"
                >
                    View Consumption
                </BaseButton>
                <BaseButton
                    @click="$router.push({ name: 'MeterReadingCreate', query: { meterId } })"
                    variant="primary"
                >
                    Add Reading
                </BaseButton>
            </template>
        </PageHeader>

        <DataCard v-if="meter" :title="meter.name" class="mb-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                    <p class="text-gray-600">
                        <span class="font-medium">Type:</span> {{ meter.meter_type }}
                    </p>
                    <p class="text-gray-600">
                        <span class="font-medium">Unit:</span> {{ meter.unit }}
                    </p>
                    <p class="text-gray-600">
                        <span class="font-medium">Assignment:</span>
                        {{ meter.assignment_type === 'unit' ? 'Property Unit' : 'Common Area' }}
                        <span v-if="meter.assignment_type === 'unit' && propertyUnit">
                            ({{ propertyUnit.name }})
                        </span>
                    </p>
                </div>
                <div>
                    <div class="bg-blue-50 text-blue-700 p-3 rounded-lg">
                        <p class="font-medium">Latest Reading:</p>
                        <p v-if="latestReading" class="text-lg">
                            {{ latestReading.value.toFixed(2) }} {{ meter.unit }}
                        </p>
                        <p v-if="latestReading" class="text-sm">
                            on {{ formatDate(latestReading.reading_date) }}
                        </p>
                        <p v-else class="text-gray-500">No readings yet</p>
                    </div>
                </div>
            </div>
        </DataCard>

        <LoadingState
            v-if="loading"
            message="Loading readings..."
        />

        <AlertState
            v-else-if="error"
            type="error"
            :message="error"
        />

        <EmptyState
            v-else-if="!readings.length"
            title="No Readings Found"
            message="No readings have been recorded for this meter yet."
        >
            <template #actions>
                <BaseButton
                    @click="$router.push({ name: 'MeterReadingCreate', query: { meterId } })"
                    variant="primary"
                >
                    Add First Reading
                </BaseButton>
            </template>
        </EmptyState>

        <BaseTable
            v-else
            :columns="tableColumns"
            :data="readings"
            class="mt-6"
        >
            <template #cell-reading_date="{ item }">
                {{ formatDate(item.reading_date) }}
            </template>
            <template #cell-value="{ item }">
                {{ item.value.toFixed(2) }} {{ meter.unit }}
            </template>
            <template #cell-consumption="{ item, index }">
                <span v-if="index < readings.length - 1">
                    {{ (item.value - readings[index + 1].value).toFixed(2) }} {{ meter.unit }}
                </span>
                <span v-else class="text-gray-500">-</span>
            </template>
            <template #cell-notes="{ item }">
                {{ item.notes || '-' }}
            </template>
            <template #actions="{ item }">
                <BaseButton
                    @click="handleTableAction('edit', item)"
                    variant="secondary"
                    size="sm"
                    class="text-xs"
                >
                    Edit
                </BaseButton>
                <BaseButton
                    @click="handleTableAction('delete', item)"
                    variant="danger"
                    size="sm"
                    class="text-xs"
                >
                    Delete
                </BaseButton>
            </template>
        </BaseTable>

        <!-- Delete Confirmation Modal -->
        <div v-if="showDeleteModal" class="modal-overlay">
            <BaseCard class="max-w-md mx-4">
                <h3 class="text-lg font-semibold text-gray-800 mb-4">
                    Confirm Deletion
                </h3>
                <p class="text-gray-600 mb-6">
                    Are you sure you want to delete this meter reading? This action cannot be undone.
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
                        @click="deleteMeterReading"
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
import { meterReadingService, meterService, propertyUnitService } from '@/services/api';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  BaseTable,
  DataCard,
  LoadingState,
  AlertState,
  EmptyState
} from '@/components/base';

export default {
    name: 'MeterReadingsByMeter',
    components: {
        PageHeader,
        BaseButton,
        BaseCard,
        BaseTable,
        DataCard,
        LoadingState,
        AlertState,
        EmptyState
    },

    props: {
        meterId: {
            type: String,
            required: true
        }
    },

    data() {
        return {
            meter: null,
            propertyUnit: null,
            readings: [],
            loading: true,
            error: null,
            showDeleteModal: false,
            readingToDelete: null,
            deleting: false,
            tableColumns: [
                { key: 'reading_date', label: 'Date', align: 'left', width: '20%' },
                { key: 'value', label: 'Value', align: 'right', width: '16%' },
                { key: 'consumption', label: 'Consumption', align: 'right', width: '20%' },
                { key: 'notes', label: 'Notes', align: 'left', width: '28%' }
            ]
        };
    },

    computed: {
        latestReading() {
            if (this.readings.length === 0) return null;

            return this.readings.reduce((latest, reading) => {
                const latestDate = latest ? new Date(latest.reading_date) : new Date(0);
                const currentDate = new Date(reading.reading_date);
                return currentDate > latestDate ? reading : latest;
            }, null);
        }
    },

    methods: {
        handleTableAction(action, item) {
            switch (action) {
                case 'edit':
                    this.$router.push(`/meter-readings/${item.id}`);
                    break;
                case 'delete':
                    this.confirmDelete(item);
                    break;
            }
        },
        async fetchMeter() {
            try {
                const response = await meterService.get(this.meterId);
                this.meter = response.data;

                // If this is a unit meter, fetch the property unit
                if (this.meter.assignment_type === 'unit' && this.meter.property_unit_id) {
                    await this.fetchPropertyUnit(this.meter.property_unit_id);
                }
            } catch (err) {
                console.error('Error fetching meter:', err);
                this.error = 'Failed to load meter information. Please try again.';
            }
        },

        async fetchPropertyUnit(propertyUnitId) {
            try {
                const response = await propertyUnitService.get(propertyUnitId);
                this.propertyUnit = response.data;
            } catch (err) {
                console.error('Error fetching property unit:', err);
            }
        },

        async fetchReadings() {
            try {
                const response = await meterReadingService.getByMeter(this.meterId);
                // Sort readings by date (newest first) for proper consumption calculation
                this.readings = response.data.sort((a, b) =>
                    new Date(b.reading_date) - new Date(a.reading_date)
                );
                this.error = null;
            } catch (err) {
                console.error('Error fetching meter readings:', err);
                this.error = 'Failed to load meter readings. Please try again.';
            } finally {
                this.loading = false;
            }
        },

        formatDate(dateString) {
            return new Date(dateString).toLocaleDateString();
        },

        confirmDelete(reading) {
            this.readingToDelete = reading;
            this.showDeleteModal = true;
        },

        async deleteMeterReading() {
            if (!this.readingToDelete) return;

            this.deleting = true;
            try {
                await meterReadingService.delete(this.readingToDelete.id);
                this.readings = this.readings.filter(r => r.id !== this.readingToDelete.id);
                this.showDeleteModal = false;
                this.readingToDelete = null;
            } catch (err) {
                console.error('Error deleting meter reading:', err);
                this.error = 'Failed to delete meter reading. Please try again.';
            } finally {
                this.deleting = false;
            }
        }
    },

    async created() {
        await this.fetchMeter();
        await this.fetchReadings();
    }
};
</script>

<style scoped>
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
}
</style>
