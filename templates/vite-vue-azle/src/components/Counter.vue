<script setup lang="ts">
  import {ref} from 'vue';
  import { backend } from "../declarations/backend/index.js"
  
  const count = ref(BigInt(0))

  const refreshCounter = async () => {
    const freshCount = await backend.get()
    count.value = freshCount
  }

  const increment = async () => {
    const freshCount = await backend.inc()
    count.value = freshCount
  }

  refreshCounter()
</script>

<template>
  <div class="example">
  <button @click="increment">
    Count is {{ count }}
  </button>
</div>
</template>

