<template>
  <div>
    <h1 class="text-3xl font-bold text-gray-800 mb-6">
      {{ isEditing ? 'Edit Tenant' : 'Create Tenant' }}
    </h1>

    <div class="card max-w-2xl mx-auto">
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
            :class="{ 'border-red-500': errors.name }"
            required
          >
          <p
            v-if="errors.name"
            class="mt-1 text-sm text-red-600"
          >
            {{ errors.name }}
          </p>
        </div>

        <div class="mb-4">
          <label
            for="number_of_persons"
            class="form-label"
          >Number of Persons</label>
          <input
            id="number_of_persons"
            v-model.number="form.number_of_persons"
            type="number"
            min="1"
            step="1"
            class="form-input"
            :class="{ 'border-red-500': errors.number_of_persons }"
            required
          >
          <p
            v-if="errors.number_of_persons"
            class="mt-1 text-sm text-red-600"
          >
            {{ errors.number_of_persons }}
          </p>
        </div>

        <div class="mb-6">
          <label
            for="property_unit_id"
            class="form-label"
          >Property Unit</label>
          <select
            id="property_unit_id"
            v-model="form.property_unit_id"
            class="form-input"
            :class="{ 'border-red-500': errors.property_unit_id }"
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
            class="mt-1 text-sm text-red-600"
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
          <router-link
            to="/tenants"
            class="btn btn-secondary"
          >
            Cancel
          </router-link>
          <button
            type="submit"
            class="btn btn-primary"
            :disabled="loading || propertyUnits.length === 0"
          >
            {{ isEditing ? 'Update' : 'Create' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { tenantService, propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'TenantForm',
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
        number_of_persons: 1,
        property_unit_id: null
      },
      errors: {
        name: null,
        number_of_persons: null,
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
  async mounted() {
    await this.fetchPropertyUnits();

    if (this.isEditing) {
      await this.fetchTenant();
    }
  },
  methods: {
    async fetchPropertyUnits() {
      try {
        const response = await propertyUnitService.getAll();
        this.propertyUnits = response.data;

        // If there's only one property unit, select it by default for new tenants
        if (!this.isEditing && this.propertyUnits.length === 1) {
          this.form.property_unit_id = this.propertyUnits[0].id;
        }
      } catch (error) {
        this.toast.error('Failed to load property units');
        console.error('Error fetching property units:', error);
      }
    },
    async fetchTenant() {
      try {
        this.loading = true;
        const response = await tenantService.get(this.id);
        const tenant = response.data;

        this.form.name = tenant.name;
        this.form.number_of_persons = tenant.number_of_persons;
        this.form.property_unit_id = tenant.property_unit_id;

        this.loading = false;
      } catch (error) {
        this.toast.error('Failed to load tenant');
        this.loading = false;
        console.error('Error fetching tenant:', error);
      }
    },
    validateForm() {
      let isValid = true;
      this.errors = {
        name: null,
        number_of_persons: null,
        property_unit_id: null
      };

      if (!this.form.name || this.form.name.trim() === '') {
        this.errors.name = 'Name is required';
        isValid = false;
      }

      if (!this.form.number_of_persons) {
        this.errors.number_of_persons = 'Number of persons is required';
        isValid = false;
      } else if (this.form.number_of_persons <= 0) {
        this.errors.number_of_persons = 'Number of persons must be greater than 0';
        isValid = false;
      }

      if (!this.form.property_unit_id) {
        this.errors.property_unit_id = 'Property unit is required';
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

        if (this.isEditing) {
          await tenantService.update(this.id, this.form);
          this.toast.success('Tenant updated successfully');
        } else {
          await tenantService.create(this.form);
          this.toast.success('Tenant created successfully');
        }

        this.loading = false;
        this.$router.push('/tenants');
      } catch (error) {
        const action = this.isEditing ? 'update' : 'create';
        this.toast.error(`Failed to ${action} tenant`);
        this.loading = false;
        console.error(`Error ${action}ing tenant:`, error);

        // Handle validation errors from the backend
        if (error.response && error.response.data) {
          const errorMessage = error.response.data;
          if (errorMessage.includes('name')) {
            this.errors.name = errorMessage;
          } else if (errorMessage.includes('persons')) {
            this.errors.number_of_persons = errorMessage;
          } else if (errorMessage.includes('property unit')) {
            this.errors.property_unit_id = errorMessage;
          }
        }
      }
    }
  }
};
</script>
