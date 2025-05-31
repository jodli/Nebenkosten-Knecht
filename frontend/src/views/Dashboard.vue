<template>
  <div class="dashboard-view">
    <PageHeader title="Dashboard">
      <template #subtitle>
        Overview of your property management
      </template>
    </PageHeader>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <!-- Stats cards will go here -->
      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Properties</h3>
        <p class="text-3xl font-bold text-blue-600">0</p>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Tenants</h3>
        <p class="text-3xl font-bold text-green-600">0</p>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Meters</h3>
        <p class="text-3xl font-bold text-purple-600">0</p>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-2">Cost Types</h3>
        <p class="text-3xl font-bold text-orange-600">0</p>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Recent activity will go here -->
      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Recent Activity</h3>
        <p class="text-gray-500">No recent activity</p>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Quick Actions</h3>
        <div class="space-y-2">
          <router-link to="/property-units" class="block text-blue-600 hover:text-blue-800">
            Manage Properties
          </router-link>
          <router-link to="/tenants" class="block text-blue-600 hover:text-blue-800">
            Manage Tenants
          </router-link>
          <router-link to="/meters" class="block text-blue-600 hover:text-blue-800">
            Manage Meters
          </router-link>
          <router-link to="/cost-types" class="block text-blue-600 hover:text-blue-800">
            Manage Cost Types
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import PageHeader from '@/components/base/PageHeader.vue';

export default {
  name: 'DashboardView',
  components: {
    PageHeader
  }
};
</script>
