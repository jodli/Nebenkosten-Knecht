<template>
  <div class="container mx-auto px-4 py-8">
    <div class="mb-6">
      <button @click="$router.go(-1)" class="text-blue-500 hover:text-blue-700">
        &larr; Back to Billing Periods
      </button>
      <h1 class="text-2xl font-bold mt-2">Billing Statements for {{ periodName }}</h1>
      <p class="text-gray-600">{{ formatDateRange() }}</p>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-8">
      <div class="spinner"></div>
      <p class="mt-2">Loading...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <p>{{ error }}</p>
    </div>

    <!-- Main Content -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column: Tenant List -->
      <div class="lg:col-span-1 bg-white p-6 rounded-lg shadow">
        <h2 class="text-lg font-bold mb-4">Tenants</h2>

        <div v-if="tenants.length === 0" class="text-gray-500">
          No tenants found for this property unit.
        </div>

        <ul v-else>
          <li
            v-for="tenant in tenants"
            :key="tenant.id"
            @click="selectTenant(tenant)"
            :class="[
              'p-3 mb-2 border rounded cursor-pointer hover:bg-gray-100',
              selectedTenant && selectedTenant.id === tenant.id ? 'bg-blue-100 border-blue-500' : 'border-gray-200'
            ]"
          >
            <div class="font-medium">{{ tenant.name }}</div>
            <div class="text-sm text-gray-600">
              {{ tenant.persons }} person{{ tenant.persons !== 1 ? 's' : '' }} |
              {{ tenant.area }} m²
            </div>
            <div v-if="tenantHasStatement(tenant.id)" class="mt-1 text-xs">
              <span class="bg-green-100 text-green-800 px-2 py-1 rounded">Statement Generated</span>
            </div>
          </li>
        </ul>

        <button
          v-if="selectedTenant"
          @click="generateStatement"
          class="w-full mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
          :disabled="generating"
        >
          {{ generating ? 'Generating...' : (tenantHasStatement(selectedTenant.id) ? 'Regenerate Statement' : 'Generate Statement') }}
        </button>
      </div>

      <!-- Right Column: Statement Preview -->
      <div class="lg:col-span-2 bg-white rounded-lg shadow overflow-hidden">
        <div v-if="!selectedTenant || (!selectedStatement && !generating)" class="p-6 text-center text-gray-500 flex flex-col items-center justify-center h-full">
          <svg class="w-16 h-16 mb-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
          </svg>
          <p>Select a tenant and generate a statement to preview it here.</p>
        </div>

        <div v-else-if="generating && !selectedStatement" class="p-6 text-center flex flex-col items-center justify-center h-full">
          <div class="spinner mb-4"></div>
          <p>Generating statement...</p>
          <p class="text-sm text-gray-500 mt-2">This may take a moment while we calculate all costs.</p>
        </div>

        <div v-else-if="selectedStatement" class="flex flex-col h-full">
          <div class="flex justify-between items-center p-4 border-b">
            <h3 class="font-bold">
              Statement for {{ selectedTenant.name }}
            </h3>
            <div class="flex space-x-2">
              <button
                @click="printStatement"
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-3 rounded text-sm flex items-center"
              >
                <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"></path>
                </svg>
                Print
              </button>
            </div>
          </div>

          <div class="flex-grow overflow-auto p-4" v-html="statementHtml"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import billingService from '@/services/billingService';
import { tenantService } from '@/services/api';

export default {
  name: 'BillingStatementList',
  props: {
    periodId: {
      type: [String, Number],
      required: true
    }
  },
  data() {
    return {
      billingPeriod: null,
      tenants: [],
      statements: [],
      selectedTenant: null,
      selectedStatement: null,
      statementHtml: '',
      loading: true,
      error: null,
      generating: false
    };
  },
  computed: {
    periodName() {
      return this.billingPeriod ? this.billingPeriod.name : '';
    }
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      this.loading = true;
      this.error = null;

      try {
        // Get billing period details
        const periodResponse = await billingService.getBillingPeriod(this.periodId);
        this.billingPeriod = periodResponse.data;

        // Get tenants for this property unit
        const tenantResponse = await tenantService.getByPropertyUnit(this.billingPeriod.property_unit_id);
        this.tenants = tenantResponse.data;

        // Get any existing statements for this period
        // We'll need to collect all tenant statements, one by one
        this.statements = [];
        for (const tenant of this.tenants) {
          try {
            const statementResponse = await billingService.getTenantStatements(tenant.id);
            const tenantStatements = statementResponse.data.filter(
              s => s.billing_period_id === parseInt(this.periodId)
            );
            this.statements = [...this.statements, ...tenantStatements];
          } catch (error) {
            console.warn(`Could not fetch statements for tenant ${tenant.id}`, error);
          }
        }
      } catch (error) {
        console.error('Error fetching data:', error);
        this.error = 'Failed to load data. Please try again later.';
      } finally {
        this.loading = false;
      }
    },

    formatDateRange() {
      if (!this.billingPeriod) return '';

      const startDate = new Date(this.billingPeriod.start_date);
      const endDate = new Date(this.billingPeriod.end_date);

      return `${startDate.toLocaleDateString()} - ${endDate.toLocaleDateString()}`;
    },

    tenantHasStatement(tenantId) {
      return this.statements.some(s => s.tenant_id === tenantId);
    },

    selectTenant(tenant) {
      this.selectedTenant = tenant;

      // If this tenant already has a statement, load it
      const existingStatement = this.statements.find(s => s.tenant_id === tenant.id);
      if (existingStatement) {
        this.selectedStatement = existingStatement;
        this.loadStatementHtml(existingStatement.id);
      } else {
        this.selectedStatement = null;
        this.statementHtml = '';
      }
    },

    async loadStatementHtml(statementId) {
      try {
        const response = await billingService.getStatementHtml(statementId);
        this.statementHtml = response.data;
      } catch (error) {
        console.error('Error loading statement HTML:', error);
        this.error = 'Failed to load statement content.';
      }
    },

    async generateStatement() {
      if (!this.selectedTenant) return;

      this.generating = true;
      this.error = null;

      try {
        const response = await billingService.generateStatement(
          parseInt(this.periodId),
          this.selectedTenant.id
        );

        // Update the statements list
        const existingIndex = this.statements.findIndex(
          s => s.tenant_id === this.selectedTenant.id &&
               s.billing_period_id === parseInt(this.periodId)
        );

        if (existingIndex >= 0) {
          this.statements[existingIndex] = response.data;
        } else {
          this.statements.push(response.data);
        }

        // Set as selected and load HTML
        this.selectedStatement = response.data;
        await this.loadStatementHtml(response.data.id);
      } catch (error) {
        console.error('Error generating statement:', error);
        if (error.response && error.response.data) {
          this.error = error.response.data;
        } else {
          this.error = 'Failed to generate statement. Please try again.';
        }
      } finally {
        this.generating = false;
      }
    },

    printStatement() {
      if (!this.statementHtml) return;

      // Create a new window for printing
      const printWindow = window.open('', '_blank');

      // Write HTML content without using string literals that confuse Vue's template parser
      printWindow.document.write('<!DOCTYPE html><html>');
      printWindow.document.write('<head><title>Nebenkostenabrechnung</title></head>');
      printWindow.document.write('<body>');
      printWindow.document.write(this.statementHtml);

      // Add script for printing and auto-closing
      const printScript = 'window.onload = function() { window.print(); setTimeout(function() { window.close(); }, 500); };';
      const scriptTag = document.createElement('script');
      scriptTag.textContent = printScript;

      printWindow.document.body.appendChild(scriptTag);
      printWindow.document.write('</body></html>');
      printWindow.document.close();
    }
  }
}
</script>

<style scoped>
.spinner {
  border: 4px solid rgba(0, 0, 0, 0.1);
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border-left-color: #3490dc;
  animation: spin 1s linear infinite;
  margin: 0 auto;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
