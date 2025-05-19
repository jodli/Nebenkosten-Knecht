<template>
    <div>
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold text-gray-800">
                Consumption for {{ meter ? meter.name : 'Loading...' }}
            </h1>
            <div class="flex space-x-3">
                <router-link :to="{ name: 'MeterReadingsByMeter', params: { meterId } }" class="btn btn-secondary">
                    View Readings
                </router-link>
                <router-link :to="{ name: 'MeterReadingCreate', query: { meterId } }" class="btn btn-primary">
                    Add Reading
                </router-link>
            </div>
        </div>

        <div v-if="meter" class="card p-4 mb-6">
            <div class="flex flex-col md:flex-row md:justify-between">
                <div>
                    <h2 class="text-lg font-semibold text-gray-800">
                        {{ meter.name }}
                    </h2>
                    <p class="text-gray-600">
                        Type: {{ meter.meter_type }} | Unit: {{ meter.unit }}
                    </p>
                    <p class="text-gray-600">
                        Assignment: {{ meter.assignment_type === 'unit' ? 'Property Unit' : 'Common Area' }}
                        <span v-if="meter.assignment_type === 'unit' && propertyUnit">
                            ({{ propertyUnit.name }})
                        </span>
                    </p>
                </div>
                <div class="mt-3 md:mt-0">
                    <div class="text-sm bg-blue-50 text-blue-700 px-3 py-2 rounded-lg" v-if="totalConsumption !== null">
                        <p><strong>Total Consumption:</strong> {{ totalConsumption.toFixed(2) }} {{ meter.unit }}</p>
                        <p><strong>Period:</strong> {{ consumptionPeriod }}</p>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="loading" class="text-center py-8">
            <p class="text-gray-600">
                Loading consumption data...
            </p>
        </div>

        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            {{ error }}
        </div>

        <div v-else-if="readings.length < 2" class="card text-center py-8">
            <p class="text-gray-600" v-if="readings.length === 0">
                No readings found for this meter. At least two readings are required to calculate consumption.
            </p>
            <p class="text-gray-600" v-else>
                Only one reading found. At least two readings are required to calculate consumption.
            </p>
            <div class="mt-4">
                <router-link :to="{ name: 'MeterReadingCreate', query: { meterId } }" class="btn btn-primary">
                    Add Reading
                </router-link>
            </div>
        </div>

        <div v-else>
            <div class="mb-4 flex flex-col md:flex-row md:items-center md:justify-between">
                <h2 class="text-xl font-semibold text-gray-800 mb-2 md:mb-0">
                    Consumption History
                </h2>
                <div class="flex flex-wrap gap-2">
                    <button @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'" class="btn btn-outline">
                        Sort: {{ sortOrder === 'asc' ? 'Oldest First' : 'Newest First' }}
                    </button>
                    <button @click="showDailyConsumption = !showDailyConsumption" class="btn btn-outline">
                        {{ showDailyConsumption ? 'Show Total Consumption' : 'Show Daily Average' }}
                    </button>
                </div>
            </div>

            <div class="card overflow-x-auto">
                <table class="min-w-full bg-white">
                    <thead>
                        <tr class="bg-gray-100 text-gray-600 uppercase text-sm leading-normal">
                            <th class="py-3 px-6 text-left">Reading Date</th>
                            <th class="py-3 px-6 text-right">Meter Value</th>
                            <th class="py-3 px-6 text-right">Consumption</th>
                            <th class="py-3 px-6 text-center">Period</th>
                            <th v-if="showDailyConsumption" class="py-3 px-6 text-right">Daily Average</th>
                            <th class="py-3 px-6 text-left">Notes</th>
                        </tr>
                    </thead>
                    <tbody class="text-gray-600 text-sm">
                        <tr v-for="reading in sortedReadings" :key="reading.id"
                            class="border-b border-gray-200 hover:bg-gray-50">
                            <td class="py-3 px-6 text-left">
                                {{ formatDate(reading.reading_date) }}
                            </td>
                            <td class="py-3 px-6 text-right">
                                {{ reading.value.toFixed(2) }} {{ meter.unit }}
                            </td>
                            <td class="py-3 px-6 text-right">
                                <span v-if="reading.consumption !== null"
                                    :class="{ 'text-green-600 font-semibold': reading.consumption > 0 }">
                                    {{ reading.consumption.toFixed(2) }} {{ meter.unit }}
                                </span>
                                <span v-else class="text-gray-400">-</span>
                            </td>
                            <td class="py-3 px-6 text-center">
                                <span v-if="reading.days_since_last_reading !== null">
                                    {{ reading.days_since_last_reading }} days
                                </span>
                                <span v-else class="text-gray-400">-</span>
                            </td>
                            <td v-if="showDailyConsumption" class="py-3 px-6 text-right">
                                <span v-if="reading.consumption !== null && reading.days_since_last_reading > 0">
                                    {{ (reading.consumption / reading.days_since_last_reading).toFixed(3) }} {{
                                    meter.unit }}/day
                                </span>
                                <span v-else class="text-gray-400">-</span>
                            </td>
                            <td class="py-3 px-6 text-left">
                                {{ reading.notes || '-' }}
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <!-- Consumption Chart -->
            <div class="mt-6">
                <h2 class="text-xl font-semibold text-gray-800 mb-4">
                    Consumption Chart
                </h2>
                <div class="card p-4">
                    <p class="text-center text-gray-500 italic">
                        Chart feature will be implemented in a future update
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { meterReadingService, meterService, propertyUnitService } from '@/services/api';

export default {
    name: 'MeterConsumption',

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

<style scoped>
.card {
    @apply bg-white shadow rounded-lg p-4;
}

.btn {
    @apply px-4 py-2 rounded-md text-sm font-medium;
}

.btn-primary {
    @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-secondary {
    @apply bg-gray-200 text-gray-800 hover:bg-gray-300;
}

.btn-outline {
    @apply border border-gray-300 text-gray-700 hover:bg-gray-50;
}
</style>
