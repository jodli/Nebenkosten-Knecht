<template>
  <div class="empty-state">
    <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-gray-100">
      <slot name="icon">
        <svg class="h-6 w-6 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
        </svg>
      </slot>
    </div>

    <h3 v-if="title" class="mt-2 text-sm font-medium text-gray-900">
      {{ title }}
    </h3>

    <p class="empty-text">
      {{ message }}
    </p>

    <div v-if="$slots.actions" class="mt-6">
      <slot name="actions"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'EmptyState',
  props: {
    title: {
      type: String,
      default: null
    },
    message: {
      type: String,
      required: true
    }
  }
};
</script>
