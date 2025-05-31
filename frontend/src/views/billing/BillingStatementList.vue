<template>
  <div>
    <PageHeader :title="`Billing Statements for ${periodName}`">
      <template #subtitle>
        {{ formatDateRange() }}
      </template>
      <template #actions>
        <BaseButton
          @click="$router.go(-1)"
          variant="secondary"
        >
          ← Back to Billing Periods
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

    <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column: Tenant List -->
      <BaseCard class="lg:col-span-1">
        <h2 class="text-lg font-semibold text-gray-800 mb-4">Tenants</h2>

        <EmptyState
          v-if="tenants.length === 0"
          title="No Tenants Found"
          message="No tenants found for this property unit."
          size="sm"
        />

        <div v-else class="space-y-2">
          <div
            v-for="tenant in tenants"
            :key="tenant.id"
            @click="selectTenant(tenant)"
            :class="[
              'p-3 border rounded cursor-pointer hover:bg-gray-50 transition-colors',
              selectedTenant && selectedTenant.id === tenant.id ? 'bg-blue-50 border-blue-300' : 'border-gray-200'
            ]"
          >
            <div class="font-medium text-gray-800">{{ tenant.name }}</div>
            <div class="text-sm text-gray-600">
              {{ tenant.persons }} person{{ tenant.persons !== 1 ? 's' : '' }} |
              {{ tenant.area }} m²
            </div>
            <div v-if="tenantHasStatement(tenant.id)" class="mt-1">
              <span class="text-xs bg-green-100 text-green-800 px-2 py-1 rounded">Statement Generated</span>
            </div>
          </div>

          <BaseButton
            v-if="selectedTenant"
            @click="generateStatement"
            variant="primary"
            class="w-full mt-4"
            :disabled="generating"
          >
            {{ generating ? 'Generating...' : (tenantHasStatement(selectedTenant.id) ? 'Regenerate Statement' : 'Generate Statement') }}
          </BaseButton>
        </div>
      </BaseCard>

      <!-- Right Column: Statement Preview -->
      <BaseCard class="lg:col-span-2 min-h-[400px]">
        <EmptyState
          v-if="!selectedTenant || (!selectedStatement && !generating)"
          title="No Statement Selected"
          message="Select a tenant and generate a statement to preview it here."
          size="lg"
        />

        <LoadingState
          v-else-if="generating && !selectedStatement"
          message="Generating statement..."
          subtitle="This may take a moment while we calculate all costs."
        />

        <div v-else-if="selectedStatement" class="flex flex-col h-full">
          <div class="flex justify-between items-center pb-4 border-b border-gray-200 mb-4">
            <h3 class="text-lg font-semibold text-gray-800">
              Statement for {{ selectedTenant.name }}
            </h3>
            <BaseButton
              @click="printStatement"
              variant="primary"
              size="sm"
            >
              <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"></path>
              </svg>
              Print
            </BaseButton>
          </div>

          <div class="flex-grow overflow-auto" v-html="statementHtml"></div>
        </div>
      </BaseCard>
    </div>
  </div>
</template>

<script>
import billingService from '@/services/billingService';
import { tenantService } from '@/services/api';
import {
  PageHeader,
  BaseButton,
  BaseCard,
  LoadingState,
  AlertState,
  EmptyState
} from '@/components/base';

export default {
  name: 'BillingStatementList',
  components: {
    PageHeader,
    BaseButton,
    BaseCard,
    LoadingState,
    AlertState,
    EmptyState
  },
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
