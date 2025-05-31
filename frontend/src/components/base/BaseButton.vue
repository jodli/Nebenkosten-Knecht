<template>
  <button
    :class="buttonClasses"
    :disabled="disabled"
    :type="type"
    @click="$emit('click', $event)"
  >
    <slot></slot>
  </button>
</template>

<script>
export default {
  name: 'BaseButton',
  emits: ['click'],
  props: {
    variant: {
      type: String,
      default: 'primary',
      validator: (value) => ['primary', 'secondary', 'success', 'warning', 'danger', 'outline'].includes(value)
    },
    size: {
      type: String,
      default: 'md',
      validator: (value) => ['sm', 'md', 'lg'].includes(value)
    },
    disabled: {
      type: Boolean,
      default: false
    },
    type: {
      type: String,
      default: 'button'
    }
  },
  computed: {
    buttonClasses() {
      const baseClasses = 'btn';
      const variantClass = `btn-${this.variant}`;
      const sizeClass = this.size !== 'md' ? `btn-${this.size}` : '';
      const disabledClass = this.disabled ? 'opacity-50 cursor-not-allowed' : '';

      return [baseClasses, variantClass, sizeClass, disabledClass].filter(Boolean).join(' ');
    }
  }
};
</script>
