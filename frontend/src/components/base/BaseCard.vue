<template>
  <div :class="cardClasses">
    <slot></slot>
  </div>
</template>

<script>
export default {
  name: 'BaseCard',
  props: {
    size: {
      type: String,
      default: 'md',
      validator: (value) => ['sm', 'md', 'lg'].includes(value)
    },
    hover: {
      type: Boolean,
      default: false
    }
  },
  computed: {
    cardClasses() {
      const baseClasses = 'card';
      const sizeClass = this.size !== 'md' ? `card-${this.size}` : '';
      const hoverClass = this.hover ? 'card-hover' : '';

      return [baseClasses, sizeClass, hoverClass].filter(Boolean).join(' ');
    }
  }
};
</script>
