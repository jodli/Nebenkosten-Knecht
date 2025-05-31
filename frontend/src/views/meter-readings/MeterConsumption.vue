<template>
    <div>
        <PageHeader :title="`Consumption for ${meter ? meter.name : 'Loading...'}`">
            <template #actions>
                <BaseButton
                    @click="$router.push({ name: 'MeterReadingsByMeter', params: { meterId } })"
                    variant="secondary"
                >
                    View Readings
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
                <div v-if="totalConsumption !== null">
                    <div class="bg-blue-50 text-blue-700 p-3 rounded-lg">
                        <p class="font-medium">Total Consumption:</p>
                        <p class="text-lg">{{ totalConsumption.toFixed(2) }} {{ meter.unit }}</p>
                        <p class="text-sm">Period: {{ consumptionPeriod }}</p>
                    </div>
                </div>
            </div>
        </DataCard>

        <LoadingState
            v-if="loading"
            message="Loading consumption data..."
        />

        <AlertState
            v-else-if="error"
            type="error"
            :message="error"
        />

        <EmptyState
            v-else-if="readings.length < 2"
            title="Insufficient Data"
            :message="readings.length === 0 ? 'No readings found for this meter. At least two readings are required to calculate consumption.' : 'Only one reading found. At least two readings are required to calculate consumption.'"
        >
            <template #actions>
                <BaseButton
                    @click="$router.push({ name: 'MeterReadingCreate', query: { meterId } })"
                    variant="primary"
                >
                    Add Reading
                </BaseButton>
            </template>
        </EmptyState>

        <div v-else>
            <BaseCard class="mb-6">
                <div class="flex flex-col md:flex-row md:items-center md:justify-between mb-4">
                    <h2 class="text-xl font-semibold text-gray-800 mb-2 md:mb-0">
                        Consumption History
                    </h2>
                    <div class="flex flex-wrap gap-2">
                        <BaseButton
                            @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'"
                            variant="outline"
                        >
                            Sort: {{ sortOrder === 'asc' ? 'Oldest First' : 'Newest First' }}
                        </BaseButton>
                        <BaseButton
                            @click="showDailyConsumption = !showDailyConsumption"
                            variant="outline"
                        >
                            {{ showDailyConsumption ? 'Show Total Consumption' : 'Show Daily Average' }}
                        </BaseButton>
                    </div>
                </div>

                <BaseTable
                    :columns="tableHeaders"
                    :data="sortedReadings"
                >
                    <template #cell-reading_date="{ item }">
                        {{ formatDate(item.reading_date) }}
                    </template>
                    <template #cell-value="{ item }">
                        {{ item.value.toFixed(2) }} {{ meter.unit }}
                    </template>
                    <template #cell-consumption="{ item }">
                        <span v-if="item.consumption !== null"
                            :class="{ 'text-green-600 font-semibold': item.consumption > 0 }">
                            {{ item.consumption.toFixed(2) }} {{ meter.unit }}
                        </span>
                        <span v-else class="text-gray-400">-</span>
                    </template>
                    <template #cell-period="{ item }">
                        <span v-if="item.days_since_last_reading !== null">
                            {{ item.days_since_last_reading }} days
                        </span>
                        <span v-else class="text-gray-400">-</span>
                    </template>
                    <template #cell-daily_average="{ item }">
                        <span v-if="item.consumption !== null && item.days_since_last_reading > 0">
                            {{ (item.consumption / item.days_since_last_reading).toFixed(3) }} {{ meter.unit }}/day
                        </span>
                        <span v-else class="text-gray-400">-</span>
                    </template>
                    <template #cell-notes="{ item }">
                        {{ item.notes || '-' }}
                    </template>
                </BaseTable>
            </BaseCard>

            <!-- Consumption Chart -->
            <BaseCard>
                <h2 class="text-xl font-semibold text-gray-800 mb-4">
                    Consumption Chart
                </h2>
                <p class="text-center text-gray-500 italic">
                    Chart feature will be implemented in a future update
                </p>
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
    name: 'MeterConsumption',
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
            sortOrder: 'asc', // Default to oldest first for consumption analysis
            showDailyConsumption: false
        };
    },

    computed: {
        tableHeaders() {
            const baseHeaders = [
                { key: 'reading_date', label: 'Reading Date' },
                { key: 'value', label: 'Meter Value' },
                { key: 'consumption', label: 'Consumption' },
                { key: 'period', label: 'Period' }
            ];

            if (this.showDailyConsumption) {
                baseHeaders.push({ key: 'daily_average', label: 'Daily Average' });
            }

            baseHeaders.push({ key: 'notes', label: 'Notes' });

            return baseHeaders;
        },

        sortedReadings() {
            return [...this.readings].sort((a, b) => {
                const dateA = new Date(a.reading_date);
                const dateB = new Date(b.reading_date);
                return this.sortOrder === 'asc'
                    ? dateA - dateB
                    : dateB - dateA;
            });
        },

        totalConsumption() {
            if (this.readings.length < 2) return null;

            // Find first and last reading based on date
            const allReadings = [...this.readings].sort((a, b) =>
                new Date(a.reading_date) - new Date(b.reading_date)
            );

            const firstReading = allReadings[0];
            const lastReading = allReadings[allReadings.length - 1];

            return lastReading.value - firstReading.value;
        },

        consumptionPeriod() {
            if (this.readings.length < 2) return '';

            // Find first and last reading based on date
            const allReadings = [...this.readings].sort((a, b) =>
                new Date(a.reading_date) - new Date(b.reading_date)
            );

            const firstReading = allReadings[0];
            const lastReading = allReadings[allReadings.length - 1];

            return `${this.formatDate(firstReading.reading_date)} - ${this.formatDate(lastReading.reading_date)}`;
        }
    },

    methods: {
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

        async fetchConsumption() {
            try {
                const response = await meterReadingService.getConsumption(this.meterId);
                this.readings = response.data;
                this.error = null;
            } catch (err) {
                console.error('Error fetching consumption data:', err);
                this.error = 'Failed to load consumption data. Please try again.';
            } finally {
                this.loading = false;
            }
        },

        formatDate(dateString) {
            return new Date(dateString).toLocaleDateString();
        }
    },

    async created() {
        await this.fetchMeter();
        await this.fetchConsumption();
    }
};
</script>
