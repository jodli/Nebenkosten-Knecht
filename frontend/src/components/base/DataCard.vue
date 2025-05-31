<template>
  <BaseCard :hover="hover">
    <div class="flex justify-between items-start">
      <div class="flex-1">
        <h3 v-if="title" class="card-title">
          {{ title }}
        </h3>
        <slot></slot>
      </div>

      <div v-if="$slots.actions" class="flex-shrink-0 ml-4">
        <div class="flex space-x-2">
          <slot name="actions"></slot>
        </div>
      </div>
    </div>
  </BaseCard>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  name: 'DataCard',
  components: {
    BaseCard
  },
  props: {
    title: {
      type: String,
      default: null
    },
    hover: {
      type: Boolean,
      default: true
    }
  }
};
</script>
