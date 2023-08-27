<script lang="ts">
  import { backend } from "../declarations/backend/index.js";
  import {AuthClient} from "@dfinity/auth-client";

  const authClient = await AuthClient.create();
  authClient.login({
  // 7 days in nanoseconds
  maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000),
  identityProvider: "https://identity.ic0.app/#authorize",
  onSuccess: async () => {
    handleAuthenticated(authClient);
  },
});

  let count : BigInt = BigInt(0);

  const refreshCounter = async () => {
    const freshCount = await backend.get()
    count = freshCount;
  }

  const increment = async () => {
    await backend.inc()
    await refreshCounter()
  }

  refreshCounter()


</script>
<div class="example">
  <button on:click={increment}>
    Count is {count}
  </button>
</div>