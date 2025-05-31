<template>
  <div>
    <PageHeader :title="isEditing ? 'Edit Meter' : 'Create Meter'" />

    <BaseCard size="lg" class="max-w-2xl mx-auto">
      <form @submit.prevent="submitForm">
        <div class="mb-4">
          <label
            for="name"
            class="form-label"
          >Name</label>
          <input
            id="name"
            v-model="form.name"
            type="text"
            class="form-input"
            :class="{ 'form-input-error': errors.name }"
            required
          >
          <p
            v-if="errors.name"
            class="form-error"
          >
            {{ errors.name }}
          </p>
        </div>

        <div class="mb-4">
          <label
            for="meter_type"
            class="form-label"
          >Meter Type</label>
          <input
            id="meter_type"
            v-model="form.meter_type"
            type="text"
            class="form-input"
            :class="{ 'form-input-error': errors.meter_type }"
            placeholder="e.g., Electricity, Water, Gas"
            required
          >
          <p
            v-if="errors.meter_type"
            class="form-error"
          >
            {{ errors.meter_type }}
          </p>
        </div>

        <div class="mb-4">
          <label
            for="unit"
            class="form-label"
          >Unit</label>
          <input
            id="unit"
            v-model="form.unit"
            type="text"
            class="form-input"
            :class="{ 'form-input-error': errors.unit }"
            placeholder="e.g., kWh, m³"
            required
          >
          <p
            v-if="errors.unit"
            class="form-error"
          >
            {{ errors.unit }}
          </p>
        </div>

        <div class="mb-4">
          <label
            for="assignment_type"
            class="form-label"
          >Assignment Type</label>
          <select
            id="assignment_type"
            v-model="form.assignment_type"
            class="form-input"
            :class="{ 'form-input-error': errors.assignment_type }"
            required
          >
            <option value="unit">
              Unit (assigned to a specific property unit)
            </option>
            <option value="common">
              Common (shared among all units)
            </option>
          </select>
          <p
            v-if="errors.assignment_type"
            class="form-error"
          >
            {{ errors.assignment_type }}
          </p>
        </div>

        <div
          v-if="form.assignment_type === 'unit'"
          class="mb-6"
        >
          <label
            for="property_unit_id"
            class="form-label"
          >Property Unit</label>
          <select
            id="property_unit_id"
            v-model="form.property_unit_id"
            class="form-input"
            :class="{ 'form-input-error': errors.property_unit_id }"
            required
          >
            <option
              :value="null"
              disabled
            >
              Select a property unit
            </option>
            <option
              v-for="unit in propertyUnits"
              :key="unit.id"
              :value="unit.id"
            >
              {{ unit.name }} ({{ unit.living_area_m2 }} m²)
            </option>
          </select>
          <p
            v-if="errors.property_unit_id"
            class="form-error"
          >
            {{ errors.property_unit_id }}
          </p>

          <div
            v-if="propertyUnits.length === 0"
            class="mt-2 text-sm text-amber-600"
          >
            No property units found. Please create a property unit first.
            <router-link
              to="/property-units/create"
              class="text-primary-600 hover:underline"
            >
              Create Property Unit
            </router-link>
          </div>
        </div>

        <div class="flex justify-between">
          <BaseButton
            variant="secondary"
            @click="$router.push('/meters')"
          >
            Cancel
          </BaseButton>
          <BaseButton
            type="submit"
            variant="primary"
            :disabled="loading || (form.assignment_type === 'unit' && propertyUnits.length === 0)"
          >
            {{ isEditing ? 'Update' : 'Create' }}
          </BaseButton>
        </div>
      </form>
    </BaseCard>
  </div>
</template>

<script>
import { meterService, propertyUnitService } from '@/services/api';
import PageHeader from '@/components/base/PageHeader.vue';
import BaseCard from '@/components/base/BaseCard.vue';
import BaseButton from '@/components/base/BaseButton.vue';
import { useToast } from 'vue-toastification';

export default {
  name: 'MeterForm',
  components: {
    PageHeader,
    BaseCard,
    BaseButton
  },
  props: {
    id: {
      type: String,
      required: false,
      default: null
    }
  },
  setup() {
    const toast = useToast();
    return { toast };
  },
  data() {
    return {
      form: {
        name: '',
        meter_type: '',
        unit: '',
        assignment_type: 'unit',
        property_unit_id: null
      },
      errors: {
        name: null,
        meter_type: null,
        unit: null,
        assignment_type: null,
        property_unit_id: null
      },
      propertyUnits: [],
      loading: false
    };
  },
  computed: {
    isEditing() {
      return !!this.id;
    }
  },
  watch: {
    'form.assignment_type': function(newValue) {
      if (newValue === 'common') {
        this.form.property_unit_id = null;
      } else if (newValue === 'unit' && this.propertyUnits.length === 1) {
        // If there's only one property unit, select it by default
        this.form.property_unit_id = this.propertyUnits[0].id;
      }
    }
  },
  async mounted() {
    await this.fetchPropertyUnits();

    if (this.isEditing) {
      await this.fetchMeter();
    } else if (this.propertyUnits.length === 1) {
      // For new meters, if there's only one property unit, select it by default
      this.form.property_unit_id = this.propertyUnits[0].id;
    }
  },
  methods: {
    async fetchPropertyUnits() {
      try {
        const response = await propertyUnitService.getAll();
        this.propertyUnits = response.data;
      } catch (error) {
        this.toast.error('Failed to load property units');
        console.error('Error fetching property units:', error);
      }
    },
    async fetchMeter() {
      try {
        this.loading = true;
        const response = await meterService.get(this.id);
        const meter = response.data;

        this.form.name = meter.name;
        this.form.meter_type = meter.meter_type;
        this.form.unit = meter.unit;
        this.form.assignment_type = meter.assignment_type;
        this.form.property_unit_id = meter.property_unit_id;

        this.loading = false;
      } catch (error) {
        this.toast.error('Failed to load meter');
        this.loading = false;
        console.error('Error fetching meter:', error);
      }
    },
    validateForm() {
      let isValid = true;
      this.errors = {
        name: null,
        meter_type: null,
        unit: null,
        assignment_type: null,
        property_unit_id: null
      };

      if (!this.form.name || this.form.name.trim() === '') {
        this.errors.name = 'Name is required';
        isValid = false;
      }

      if (!this.form.meter_type || this.form.meter_type.trim() === '') {
        this.errors.meter_type = 'Meter type is required';
        isValid = false;
      }

      if (!this.form.unit || this.form.unit.trim() === '') {
        this.errors.unit = 'Unit is required';
        isValid = false;
      }

      if (!this.form.assignment_type) {
        this.errors.assignment_type = 'Assignment type is required';
        isValid = false;
      }

      if (this.form.assignment_type === 'unit' && !this.form.property_unit_id) {
        this.errors.property_unit_id = 'Property unit is required for unit-assigned meters';
        isValid = false;
      }

      return isValid;
    },
    async submitForm() {
      if (!this.validateForm()) {
        return;
      }

      try {
        this.loading = true;

        // Prepare data for API
        const data = {
          name: this.form.name,
          meter_type: this.form.meter_type,
          unit: this.form.unit,
          assignment_type: this.form.assignment_type,
          property_unit_id: this.form.assignment_type === 'unit' ? this.form.property_unit_id : null
        };

        if (this.isEditing) {
          await meterService.update(this.id, data);
          this.toast.success('Meter updated successfully');
        } else {
          await meterService.create(data);
          this.toast.success('Meter created successfully');
        }

        this.loading = false;
        this.$router.push('/meters');
      } catch (error) {
        const action = this.isEditing ? 'update' : 'create';
        this.toast.error(`Failed to ${action} meter`);
        this.loading = false;
        console.error(`Error ${action}ing meter:`, error);

        // Handle validation errors from the backend
        if (error.response && error.response.data) {
          const errorMessage = error.response.data;

          if (errorMessage.includes('name')) {
            this.errors.name = errorMessage;
          } else if (errorMessage.includes('type')) {
            this.errors.meter_type = errorMessage;
          } else if (errorMessage.includes('unit')) {
            this.errors.unit = errorMessage;
          } else if (errorMessage.includes('assignment')) {
            this.errors.assignment_type = errorMessage;
          } else if (errorMessage.includes('property unit')) {
            this.errors.property_unit_id = errorMessage;
          }
        }
      }
    }
  }
};
</script>
