import { createRouter, createWebHistory } from 'vue-router';

const routes = [
    {
        path: '/',
        name: 'Dashboard',
        component: () => import('@/views/Dashboard.vue')
    },
    {
        path: '/property-units',
        name: 'PropertyUnits',
        component: () => import('@/views/property-units/PropertyUnitList.vue')
    },
    {
        path: '/property-units/create',
        name: 'PropertyUnitCreate',
        component: () => import('@/views/property-units/PropertyUnitForm.vue')
    },
    {
        path: '/property-units/:id',
        name: 'PropertyUnitEdit',
        component: () => import('@/views/property-units/PropertyUnitForm.vue'),
        props: true
    },
    {
        path: '/tenants',
        name: 'Tenants',
        component: () => import('@/views/tenants/TenantList.vue')
    },
    {
        path: '/tenants/create',
        name: 'TenantCreate',
        component: () => import('@/views/tenants/TenantForm.vue')
    },
    {
        path: '/tenants/:id',
        name: 'TenantEdit',
        component: () => import('@/views/tenants/TenantForm.vue'),
        props: true
    },
    {
        path: '/meters',
        name: 'Meters',
        component: () => import('@/views/meters/MeterList.vue')
    },
    {
        path: '/meters/create',
        name: 'MeterCreate',
        component: () => import('@/views/meters/MeterForm.vue')
    },
    {
        path: '/meters/:id',
        name: 'MeterEdit',
        component: () => import('@/views/meters/MeterForm.vue'),
        props: true
    },
    {
        path: '/meter-readings',
        name: 'MeterReadings',
        component: () => import('@/views/meter-readings/MeterReadingList.vue')
    },
    {
        path: '/meter-readings/create',
        name: 'MeterReadingCreate',
        component: () => import('@/views/meter-readings/MeterReadingForm.vue')
    },
    {
        path: '/meter-readings/:id',
        name: 'MeterReadingEdit',
        component: () => import('@/views/meter-readings/MeterReadingForm.vue'),
        props: true
    },
    {
        path: '/meter-readings/meter/:meterId',
        name: 'MeterReadingsByMeter',
        component: () => import('@/views/meter-readings/MeterReadingsByMeter.vue'),
        props: true
    },
    {
        path: '/meter-readings/consumption/:meterId',
        name: 'MeterConsumption',
        component: () => import('@/views/meter-readings/MeterConsumption.vue'),
        props: true
    },
    {
        path: '/cost-types',
        name: 'CostTypes',
        component: () => import('@/views/cost-types/CostTypeList.vue')
    },
    {
        path: '/cost-types/create',
        name: 'CostTypeCreate',
        component: () => import('@/views/cost-types/CostTypeForm.vue')
    },
    {
        path: '/cost-types/:id',
        name: 'CostTypeEdit',
        component: () => import('@/views/cost-types/CostTypeForm.vue'),
        props: true
    },
    {
        path: '/cost-types/:id/tariffs',
        name: 'CostTypeTariffs',
        component: () => import('@/views/cost-types/TariffList.vue'),
        props: true
    },
    {
        path: '/cost-types/:id/fixed-costs',
        name: 'CostTypeFixedCosts',
        component: () => import('@/views/cost-types/FixedCostList.vue'),
        props: true
    }
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
});

export default router;
