<template>
    <div>
        <PageHeader :title="isEdit ? 'Edit Meter Reading' : 'Add Meter Reading'">
            <template #actions>
                <BaseButton
                    @click="$router.push('/meter-readings')"
                    variant="secondary"
                >
                    Back to Readings
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
            <form @submit.prevent="submitForm" class="space-y-6">
                <div>
                    <label for="meter" class="form-label">Meter</label>
                    <select id="meter" v-model="formData.meter_id" class="form-input" required>
                        <option value="" disabled>Select a meter</option>
                        <option v-for="meter in meters" :key="meter.id" :value="meter.id">
                            {{ meter.name }} ({{ meter.meter_type }}, {{ meter.unit }})
                        </option>
                    </select>
                </div>

                <div>
                    <label for="reading_date" class="form-label">Reading Date</label>
                    <input
                        id="reading_date"
                        v-model="formData.reading_date"
                        type="date"
                        class="form-input"
                        :max="today"
                        required
                    />
                </div>

                <div>
                    <label for="value" class="form-label">Reading Value</label>
                    <div class="flex items-center space-x-2">
                        <input
                            id="value"
                            v-model.number="formData.value"
                            type="number"
                            min="0"
                            step="0.01"
                            class="form-input flex-1"
                            required
                        />
                        <span v-if="selectedMeter" class="text-gray-500 text-sm">
                            {{ selectedMeter.unit }}
                        </span>
                    </div>
                    <p v-if="formData.meter_id && previousReading" class="mt-1 text-sm text-gray-500">
                        Previous reading: {{ previousReading.value.toFixed(2) }} {{ selectedMeter ? selectedMeter.unit :
                        '' }}
                        on {{ formatDate(previousReading.reading_date) }}
                    </p>
                </div>

                <div>
                    <label for="notes" class="form-label">Notes</label>
                    <textarea
                        id="notes"
                        v-model="formData.notes"
                        class="form-input h-24"
                        placeholder="Optional notes about this reading"
                    ></textarea>
                </div>

                <div class="flex justify-end space-x-3">
                    <BaseButton
                        type="button"
                        variant="secondary"
                        @click="$router.push('/meter-readings')"
                    >
                        Cancel
                    </BaseButton>
                    <BaseButton
                        type="submit"
                        variant="primary"
                        :disabled="isSubmitting"
                    >
                        {{ isSubmitting ? 'Saving...' : 'Save Reading' }}
                    </BaseButton>
                </div>
            </form>
        </BaseCard>
    </div>
</template>

<script>
import { meterService, meterReadingService } from '@/services/api';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  LoadingState,
  AlertState
} from '@/components/base';

export default {
    name: 'MeterReadingForm',
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

    data() {
        return {
            formData: {
                meter_id: '',
                reading_date: this.formatDateForInput(new Date()),
                value: '',
                notes: ''
            },
            meters: [],
            previousReading: null,
            isEdit: false,
            loading: true,
            isSubmitting: false,
            error: null,
            today: this.formatDateForInput(new Date())
        };
    },

    computed: {
        selectedMeter() {
            return this.meters.find(meter => meter.id === this.formData.meter_id);
        }
    },

    methods: {
        formatDateForInput(date) {
            const d = new Date(date);
            let month = '' + (d.getMonth() + 1);
            let day = '' + d.getDate();
            const year = d.getFullYear();

            if (month.length < 2) month = '0' + month;
            if (day.length < 2) day = '0' + day;

            return [year, month, day].join('-');
        },

        formatDate(dateString) {
            return new Date(dateString).toLocaleDateString();
        },

        async loadMeterReading() {
            try {
                const response = await meterReadingService.get(this.id);
                const reading = response.data;

                this.formData = {
                    meter_id: reading.meter_id,
                    reading_date: this.formatDateForInput(reading.reading_date),
                    value: reading.value,
                    notes: reading.notes || ''
                };

                await this.fetchPreviousReading();
                this.error = null;
            } catch (err) {
                console.error('Error loading meter reading:', err);
                this.error = 'Failed to load meter reading. Please try again.';
            } finally {
                this.loading = false;
            }
        },

        async loadMeters() {
            try {
                const response = await meterService.getAll();
                this.meters = response.data;

                // If a meterId is present in the query params, select it
                const meterId = parseInt(this.$route.query.meterId);
                if (meterId && !this.formData.meter_id && this.meters.some(m => m.id === meterId)) {
                    this.formData.meter_id = meterId;
                    await this.fetchPreviousReading();
                }
            } catch (err) {
                console.error('Error loading meters:', err);
                this.error = 'Failed to load meters. Please try again.';
            }
        },

        async fetchPreviousReading() {
            if (!this.formData.meter_id) return;

            try {
                // Get all readings for this meter
                const response = await meterReadingService.getByMeter(this.formData.meter_id);
                const readings = response.data;

                if (readings.length === 0) return;

                // Sort by reading date in descending order
                readings.sort((a, b) => new Date(b.reading_date) - new Date(a.reading_date));

                // If we're editing, find the previous reading before this one
                if (this.isEdit) {
                    // Find the index of the current reading
                    const currentDate = new Date(this.formData.reading_date);
                    const currentIndex = readings.findIndex(r =>
                        new Date(r.reading_date).getTime() === currentDate.getTime() &&
                        r.id.toString() === this.id
                    );

                    // If there's a reading after this one, use it as previous
                    if (currentIndex >= 0 && currentIndex < readings.length - 1) {
                        this.previousReading = readings[currentIndex + 1];
                    }
                } else {
                    // For new readings, just take the most recent one
                    this.previousReading = readings[0];
                }
            } catch (err) {
                console.error('Error fetching previous reading:', err);
            }
        },

        async submitForm() {
            this.isSubmitting = true;
            this.error = null;

            try {
                const formData = {
                    ...this.formData,
                    value: parseFloat(this.formData.value),
                    meter_id: parseInt(this.formData.meter_id)
                };

                if (this.isEdit) {
                    await meterReadingService.update(this.id, {
                        reading_date: formData.reading_date,
                        value: formData.value,
                        notes: formData.notes === '' ? null : formData.notes
                    });
                } else {
                    await meterReadingService.create(formData);
                }

                // Redirect back to meter readings list
                this.$router.push('/meter-readings');
            } catch (err) {
                console.error('Error saving meter reading:', err);
                if (err.response && err.response.data) {
                    this.error = err.response.data;
                } else {
                    this.error = 'Failed to save meter reading. Please try again.';
                }
                this.isSubmitting = false;
            }
        }
    },

    watch: {
        'formData.meter_id': async function () {
            await this.fetchPreviousReading();
        }
    },

    async created() {
        this.isEdit = !!this.id;

        await this.loadMeters();

        if (this.isEdit) {
            await this.loadMeterReading();
        } else {
            this.loading = false;
        }
    }
};
</script>

<style scoped>
.card {
    @apply bg-white shadow rounded-lg p-6;
}

.btn {
    @apply px-4 py-2 rounded-md text-sm font-medium;
}

.btn-primary {
    @apply bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50;
}

.btn-secondary {
    @apply bg-gray-200 text-gray-800 hover:bg-gray-300;
}

.form-input,
.form-select,
.form-textarea {
    @apply block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm;
}
</style>
