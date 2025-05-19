<template>
    <div>
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold text-gray-800">
                Readings for {{ meter ? meter.name : 'Loading...' }}
            </h1>
            <div class="flex space-x-3">
                <router-link :to="{ name: 'MeterConsumption', params: { meterId } }" class="btn btn-secondary">
                    View Consumption
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
                    <div class="text-sm bg-blue-50 text-blue-700 px-3 py-2 rounded-lg">
                        <p><strong>Latest Reading:</strong>
                            <span v-if="latestReading">
                                {{ latestReading.value.toFixed(2) }} {{ meter.unit }}
                                on {{ formatDate(latestReading.reading_date) }}
                            </span>
                            <span v-else>No readings yet</span>
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="loading" class="text-center py-8">
            <p class="text-gray-600">
                Loading readings...
            </p>
        </div>

        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            {{ error }}
        </div>

        <div v-else-if="readings.length === 0" class="card text-center py-8">
            <p class="text-gray-600">
                No readings found for this meter.
            </p>
            <div class="mt-4">
                <router-link :to="{ name: 'MeterReadingCreate', query: { meterId } }" class="btn btn-primary">
                    Add First Reading
                </router-link>
            </div>
        </div>

        <div v-else>
            <div class="mb-4 flex items-center justify-between">
                <h2 class="text-xl font-semibold text-gray-800">
                    Reading History
                </h2>
                <div class="flex space-x-2">
                    <button @click="sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'" class="btn btn-outline">
                        Sort: {{ sortOrder === 'asc' ? 'Oldest First' : 'Newest First' }}
                    </button>
                </div>
            </div>

            <div class="card overflow-x-auto">
                <table class="min-w-full bg-white">
                    <thead>
                        <tr class="bg-gray-100 text-gray-600 uppercase text-sm leading-normal">
                            <th class="py-3 px-6 text-left">Reading Date</th>
                            <th class="py-3 px-6 text-right">Reading Value</th>
                            <th class="py-3 px-6 text-left">Notes</th>
                            <th class="py-3 px-6 text-center">Actions</th>
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
                            <td class="py-3 px-6 text-left">
                                {{ reading.notes || '-' }}
                            </td>
                            <td class="py-3 px-6 text-center">
                                <div class="flex item-center justify-center space-x-2">
                                    <router-link :to="`/meter-readings/${reading.id}`"
                                        class="text-blue-500 hover:text-blue-700">
                                        Edit
                                    </router-link>
                                    <button @click="confirmDelete(reading)" class="text-red-500 hover:text-red-700">
                                        Delete
                                    </button>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Confirmation Dialog -->
        <div v-if="showConfirmation" class="fixed inset-0 flex items-center justify-center z-50">
            <div class="fixed inset-0 bg-black opacity-50"></div>
            <div class="bg-white p-6 rounded-lg shadow-lg z-10 max-w-md mx-auto">
                <h3 class="text-lg font-semibold mb-4">
                    Confirm Deletion
                </h3>
                <p>
                    Are you sure you want to delete the meter reading from
                    <span class="font-semibold">{{ selectedReading ? formatDate(selectedReading.reading_date) : ''
                        }}</span>?
                    This action cannot be undone.
                </p>
                <div class="mt-6 flex justify-end space-x-3">
                    <button @click="showConfirmation = false" class="btn btn-secondary">
                        Cancel
                    </button>
                    <button @click="deleteReading" class="btn btn-danger">
                        Delete
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { meterReadingService, meterService, propertyUnitService } from '@/services/api';

export default {
    name: 'MeterReadingsByMeter',

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
            showConfirmation: false,
            selectedReading: null,
            sortOrder: 'desc' // Default to newest first
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
                this.readings = response.data;
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
            this.selectedReading = reading;
            this.showConfirmation = true;
        },

        async deleteReading() {
            try {
                await meterReadingService.delete(this.selectedReading.id);
                this.readings = this.readings.filter(r => r.id !== this.selectedReading.id);
                this.showConfirmation = false;
                this.selectedReading = null;
            } catch (err) {
                console.error('Error deleting meter reading:', err);
                this.error = 'Failed to delete meter reading. Please try again.';
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

.btn-danger {
    @apply bg-red-600 text-white hover:bg-red-700;
}

.btn-outline {
    @apply border border-gray-300 text-gray-700 hover:bg-gray-50;
}
</style>
