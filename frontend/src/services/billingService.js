// API service for billing periods
import { apiClient as api } from './api';

export default {
    // Get all billing periods
    getAllBillingPeriods() {
        return api.get('/billing-periods');
    },

    // Get a single billing period by ID
    getBillingPeriod(id) {
        return api.get(`/billing-periods/${id}`);
    },

    // Create a new billing period
    createBillingPeriod(billingPeriod) {
        return api.post('/billing-periods', billingPeriod);
    },

    // Update an existing billing period
    updateBillingPeriod(id, billingPeriod) {
        return api.put(`/billing-periods/${id}`, billingPeriod);
    },

    // Delete a billing period
    deleteBillingPeriod(id) {
        return api.delete(`/billing-periods/${id}`);
    },
    // Generate a statement for a tenant and period
    generateStatement(billingPeriodId, tenantId) {
        return api.post('/billing-statements/generate', {
            billing_period_id: billingPeriodId,
            tenant_id: tenantId
        });
    },

    // Get a statement by ID
    getStatement(id) {
        return api.get(`/billing-statements/${id}`);
    },

    // Get statements for a tenant
    getTenantStatements(tenantId) {
        return api.get(`/billing-statements/tenant/${tenantId}`);
    },

    // Get HTML statement by ID
    getStatementHtml(id) {
        return api.get(`/billing-statements/html/${id}`, {
            responseType: 'text'
        });
    }
};
