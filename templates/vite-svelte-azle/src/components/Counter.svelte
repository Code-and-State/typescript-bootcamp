<script lang="ts">
  import { backend } from "../declarations/backend/index.js";

  let count : BigInt = BigInt(0);

  const refreshCounter = async () => {
    const freshCount = await backend.get()
    count = freshCount;
  }

  const increment = async () => {
    await backend.inc()
    await refreshCounter()
  }

  refreshCounter();

</script>
<div class="example">
  <button on:click={increment}>
    Count is {count}
  </button>
</div>