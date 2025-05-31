<template>
  <div class="data-table">
    <div class="overflow-x-auto">
      <div
        class="table-grid"
        :style="`--columns: ${columns.length + ($slots.actions ? 1 : 0)}`"
      >
        <!-- Header Row -->
        <div class="table-header-row">
          <div
            v-for="column in columns"
            :key="column.key"
            class="table-header-cell"
            :class="column.align ? `text-${column.align}` : ''"
            :style="{ gridColumn: getColumnIndex(column.key) }"
          >
            {{ column.label }}
          </div>
          <div v-if="$slots.actions" class="table-header-cell text-center actions-col">
            Actions
          </div>
        </div>

        <!-- Data Rows -->
        <div
          v-for="(item, index) in data"
          :key="getItemKey(item, index)"
          class="table-data-row"
        >
          <div
            v-for="column in columns"
            :key="column.key"
            class="table-cell"
            :class="column.align ? `text-${column.align}` : ''"
            :style="{ gridColumn: getColumnIndex(column.key) }"
          >
            <slot
              :name="`cell-${column.key}`"
              :item="item"
              :value="getColumnValue(item, column.key)"
              :index="index"
            >
              {{ getColumnValue(item, column.key) }}
            </slot>
          </div>
          <div v-if="$slots.actions" class="table-cell text-center actions-col">
            <div class="flex justify-center space-x-2">
              <slot name="actions" :item="item" :index="index"></slot>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BaseTable',
  props: {
    data: {
      type: Array,
      required: true
    },
    columns: {
      type: Array,
      required: true
    },
    keyField: {
      type: String,
      default: 'id'
    }
  },
  methods: {
    getItemKey(item, index) {
      return item[this.keyField] || index;
    },
    getColumnValue(item, key) {
      return key.split('.').reduce((obj, key) => obj?.[key], item);
    },
    getColumnIndex(columnKey) {
      return this.columns.findIndex(col => col.key === columnKey) + 1;
    }
  }
};
</script>

<style scoped>
.data-table {
  @apply bg-white shadow rounded-lg border border-gray-200;
  overflow: visible;
}

.overflow-x-auto {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

.table-grid {
  display: grid;
  grid-template-columns: repeat(var(--columns), 1fr);
  min-width: 750px;
  width: 100%;
}

.table-header-row {
  display: contents;
  @apply bg-gray-50;
}

.table-data-row {
  display: contents;
}

.table-data-row:hover > .table-cell {
  @apply bg-gray-50;
}

.table-header-cell {
  @apply px-4 py-3 text-left text-xs font-semibold text-gray-700 uppercase tracking-wider border-b border-gray-200;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  background-color: rgb(249 250 251);
  border-bottom: 1px solid rgb(229 231 235);
}

.table-cell {
  @apply px-4 py-3 text-sm text-gray-900 border-b border-gray-200;
  vertical-align: top;
  overflow: hidden;
  text-overflow: ellipsis;
  background-color: white;
}

.actions-col {
  grid-column: -1;
}

/* Better spacing for action buttons */
.table-cell .flex {
  @apply gap-2;
}

/* Responsive column sizing */
@media (min-width: 768px) {
  .table-header-cell,
  .table-cell {
    @apply px-6 py-4;
  }
}

@media (max-width: 767px) {
  .table-grid {
    grid-template-columns: repeat(var(--columns), 1fr);
    min-width: 650px;
  }
}
</style>
