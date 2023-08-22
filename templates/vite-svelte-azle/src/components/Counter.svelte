<script lang="ts">
  import { backend } from '../declarations/backend/index.js';
  import { HttpAgent } from '@dfinity/agent';
  import { AuthClient } from '@dfinity/auth-client';

  const login = async () => {
    const authClient = await AuthClient.create();
    authClient.login({
      identityProvider: process.env.DFX_NETWORK == 'local' ? `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}` : 'https://identity.ic0.app/',
      maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000),
      onSuccess: async () => {},
    });

    const identity = await authClient.getIdentity();
    const principal = identity.getPrincipal().toString();
  };


</script>

<div class="example">
  <button on:click={login}> Login </button>
</div>
