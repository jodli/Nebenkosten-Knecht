import axios from 'axios';

// Create axios instance with base URL
export const apiClient = axios.create({
    baseURL: 'http://localhost:8081/api',
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
    },
    timeout: 10000
});

// Property Units API Service
export const propertyUnitService = {
    getAll() {
        return apiClient.get('/property-units');
    },
    get(id) {
        return apiClient.get(`/property-units/${id}`);
    },
    create(data) {
        return apiClient.post('/property-units', data);
    },
    update(id, data) {
        return apiClient.put(`/property-units/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/property-units/${id}`);
    }
};

// Tenants API Service
export const tenantService = {
    getAll() {
        return apiClient.get('/tenants');
    },
    get(id) {
        return apiClient.get(`/tenants/${id}`);
    },
    getByPropertyUnit(propertyUnitId) {
        return apiClient.get(`/tenants/by-property-unit/${propertyUnitId}`);
    },
    create(data) {
        return apiClient.post('/tenants', data);
    },
    update(id, data) {
        return apiClient.put(`/tenants/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/tenants/${id}`);
    }
};

// Meters API Service
export const meterService = {
    getAll() {
        return apiClient.get('/meters');
    },
    get(id) {
        return apiClient.get(`/meters/${id}`);
    },
    getByPropertyUnit(propertyUnitId) {
        return apiClient.get(`/meters/by-property-unit/${propertyUnitId}`);
    },
    getCommon() {
        return apiClient.get('/meters/common');
    },
    create(data) {
        return apiClient.post('/meters', data);
    },
    update(id, data) {
        return apiClient.put(`/meters/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/meters/${id}`);
    }
};

// Meter Readings API Service
export const meterReadingService = {
    getAll() {
        return apiClient.get('/meter-readings');
    },
    get(id) {
        return apiClient.get(`/meter-readings/${id}`);
    },
    getByMeter(meterId) {
        return apiClient.get(`/meter-readings/by-meter/${meterId}`);
    },
    getByDateRange(meterId, startDate, endDate) {
        return apiClient.get(`/meter-readings/by-date-range/${meterId}/${startDate}/${endDate}`);
    },
    getConsumption(meterId) {
        return apiClient.get(`/meter-readings/consumption/${meterId}`);
    },
    create(data) {
        return apiClient.post('/meter-readings', data);
    },
    update(id, data) {
        return apiClient.put(`/meter-readings/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/meter-readings/${id}`);
    }
};

// Cost Types API Service
export const costTypeService = {
    getAll() {
        return apiClient.get('/cost-types');
    },
    get(id) {
        return apiClient.get(`/cost-types/${id}`);
    },
    create(data) {
        return apiClient.post('/cost-types', data);
    },
    update(id, data) {
        return apiClient.put(`/cost-types/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/cost-types/${id}`);
    },
    getAllocationMethods() {
        return apiClient.get('/cost-types/allocation-methods');
    },
    assignAllocationMethod(costTypeId, methodId) {
        return apiClient.post(`/cost-types/${costTypeId}/allocation-methods/${methodId}`);
    },
    removeAllocationMethod(costTypeId, methodId) {
        return apiClient.delete(`/cost-types/${costTypeId}/allocation-methods/${methodId}`);
    }
};

// Tariffs API Service
export const tariffService = {
    getByCostType(costTypeId) {
        return apiClient.get(`/tariffs/cost-type/${costTypeId}`);
    },
    create(data) {
        return apiClient.post('/tariffs', data);
    },
    update(id, data) {
        return apiClient.put(`/tariffs/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/tariffs/${id}`);
    }
};

// Fixed Costs API Service
export const fixedCostService = {
    getByCostType(costTypeId) {
        return apiClient.get(`/fixed-costs/cost-type/${costTypeId}`);
    },
    create(data) {
        return apiClient.post('/fixed-costs', data);
    },
    update(id, data) {
        return apiClient.put(`/fixed-costs/${id}`, data);
    },
    delete(id) {
        return apiClient.delete(`/fixed-costs/${id}`);
    }
};
