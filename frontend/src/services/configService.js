import axios from 'axios';

const API_URL = 'http://localhost:8080';

export const configService = {
    getUnits: async () => {
        const response = await axios.get(`${API_URL}/api/config/units`);
        return response.data;
    },
    getCounters: async () => {
        const response = await axios.get(`${API_URL}/api/config/counters`);
        return response.data;
    },
    getCostTypes: async () => {
        const response = await axios.get(`${API_URL}/api/config/cost_types`);
        return response.data;
    },
    getHeizungParams: async () => {
        const response = await axios.get(`${API_URL}/api/config/heizung_params`);
        return response.data;
    }
};