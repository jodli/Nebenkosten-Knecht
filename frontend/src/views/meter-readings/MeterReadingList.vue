<template>
    <div>
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold text-gray-800">
                Meter Readings
            </h1>
            <router-link to="/meter-readings/create" class="btn btn-primary">
                Add Reading
            </router-link>
        </div>

        <div v-if="loading" class="text-center py-8">
            <p class="text-gray-600">
                Loading meter readings...
            </p>
        </div>

        <div v-else-if="error" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            {{ error }}
        </div>

        <div v-else-if="meterReadings.length === 0" class="card text-center py-8">
            <p class="text-gray-600">
                No meter readings found. Create your first meter reading to get started.
            </p>
            <div class="mt-4">
                <router-link to="/meter-readings/create" class="btn btn-primary">
                    Add Reading
                </router-link>
            </div>
        </div>

        <div v-else>
            <div class="mb-4">
                <div class="flex items-center space-x-2">
                    <label for="meterFilter" class="text-gray-700">Filter by Meter:</label>
                    <select id="meterFilter" v-model="selectedMeterId" class="form-select" @change="filterByMeter">
                        <option value="">All Meters</option>
                        <option v-for="meter in meters" :key="meter.id" :value="meter.id">
                            {{ meter.name }} ({{ meter.meter_type }})
                        </option>
                    </select>
                </div>
            </div>

            <div class="card overflow-x-auto">
                <table class="min-w-full bg-white">
                    <thead>
                        <tr class="bg-gray-100 text-gray-600 uppercase text-sm leading-normal">
                            <th class="py-3 px-6 text-left">Meter</th>
                            <th class="py-3 px-6 text-left">Reading Date</th>
                            <th class="py-3 px-6 text-right">Reading Value</th>
                            <th class="py-3 px-6 text-left">Notes</th>
                            <th class="py-3 px-6 text-center">Actions</th>
                        </tr>
                    </thead>
                    <tbody class="text-gray-600 text-sm">
                        <tr v-for="reading in meterReadings" :key="reading.id"
                            class="border-b border-gray-200 hover:bg-gray-50">
                            <td class="py-3 px-6 text-left">
                                <div v-if="getMeterById(reading.meter_id)">
                                    {{ getMeterById(reading.meter_id).name }}
                                    <span class="text-xs text-gray-500">
                                        ({{ getMeterById(reading.meter_id).meter_type }})
                                    </span>
                                </div>
                                <div v-else class="text-gray-400">
                                    Unknown Meter
                                </div>
                            </td>
                            <td class="py-3 px-6 text-left">
                                {{ formatDate(reading.reading_date) }}
                            </td>
                            <td class="py-3 px-6 text-right">
                                {{ reading.value.toFixed(2) }}
                                <span class="text-xs text-gray-500" v-if="getMeterById(reading.meter_id)">
                                    {{ getMeterById(reading.meter_id).unit }}
                                </span>
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
import { meterReadingService, meterService } from '@/services/api';

export default {
    name: 'MeterReadingList',

    data() {
        return {
            meterReadings: [],
            meters: [],
            loading: true,
            error: null,
            showConfirmation: false,
            selectedReading: null,
            selectedMeterId: ''
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

.form-select {
    @apply block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm;
}
</style>
