<template>
  <div class="page-header">
    <div>
      <h1 class="page-title">
        {{ title }}
      </h1>
      <p v-if="subtitle" class="text-gray-600 mt-2">
        {{ subtitle }}
      </p>
    </div>

    <div v-if="$slots.actions" class="flex space-x-3">
      <slot name="actions"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PageHeader',
  props: {
    title: {
      type: String,
      required: true
    },
    subtitle: {
      type: String,
      default: null
    }
  }
};
</script>
