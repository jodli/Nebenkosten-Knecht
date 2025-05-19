<template>
  <div>
    <h1 class="text-3xl font-bold text-gray-800 mb-6">
      {{ isEditing ? 'Edit Property Unit' : 'Create Property Unit' }}
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

        <div class="mb-6">
          <label
            for="living_area_m2"
            class="form-label"
          >Living Area (m²)</label>
          <input
            id="living_area_m2"
            v-model.number="form.living_area_m2"
            type="number"
            min="0.1"
            step="0.01"
            class="form-input"
            :class="{ 'border-red-500': errors.living_area_m2 }"
            required
          >
          <p
            v-if="errors.living_area_m2"
            class="mt-1 text-sm text-red-600"
          >
            {{ errors.living_area_m2 }}
          </p>
        </div>

        <div class="flex justify-between">
          <router-link
            to="/property-units"
            class="btn btn-secondary"
          >
            Cancel
          </router-link>
          <button
            type="submit"
            class="btn btn-primary"
            :disabled="loading"
          >
            {{ isEditing ? 'Update' : 'Create' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { propertyUnitService } from '@/services/api';
import { useToast } from 'vue-toastification';

export default {
  name: 'PropertyUnitForm',
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
        living_area_m2: null
      },
      errors: {
        name: null,
        living_area_m2: null
      },
      loading: false
    };
  },
  computed: {
    isEditing() {
      return !!this.id;
    }
  },
  async mounted() {
    if (this.isEditing) {
      await this.fetchPropertyUnit();
    }
  },
  methods: {
    async fetchPropertyUnit() {
      try {
        this.loading = true;
        const response = await propertyUnitService.get(this.id);
        const unit = response.data;

        this.form.name = unit.name;
        this.form.living_area_m2 = unit.living_area_m2;

        this.loading = false;
      } catch (error) {
        this.toast.error('Failed to load property unit');
        this.loading = false;
        console.error('Error fetching property unit:', error);
      }
    },
    validateForm() {
      let isValid = true;
      this.errors = {
        name: null,
        living_area_m2: null
      };

      if (!this.form.name || this.form.name.trim() === '') {
        this.errors.name = 'Name is required';
        isValid = false;
      }

      if (!this.form.living_area_m2) {
        this.errors.living_area_m2 = 'Living area is required';
        isValid = false;
      } else if (this.form.living_area_m2 <= 0) {
        this.errors.living_area_m2 = 'Living area must be greater than 0';
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
          await propertyUnitService.update(this.id, this.form);
          this.toast.success('Property unit updated successfully');
        } else {
          await propertyUnitService.create(this.form);
          this.toast.success('Property unit created successfully');
        }

        this.loading = false;
        this.$router.push('/property-units');
      } catch (error) {
        const action = this.isEditing ? 'update' : 'create';
        this.toast.error(`Failed to ${action} property unit`);
        this.loading = false;
        console.error(`Error ${action}ing property unit:`, error);

        // Handle validation errors from the backend
        if (error.response && error.response.data) {
          const errorMessage = error.response.data;
          if (errorMessage.includes('name')) {
            this.errors.name = errorMessage;
          } else if (errorMessage.includes('living area')) {
            this.errors.living_area_m2 = errorMessage;
          }
        }
      }
    }
  }
};
</script>
