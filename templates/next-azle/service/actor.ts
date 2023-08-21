import {
  createActor as azleActor,
  canisterId as azleCanisterId,
} from "@/config/declarations/dfx_generated";
import { _SERVICE } from "@/config/declarations/dfx_generated/azle_hello_world.did";
import { ActorSubclass, Identity } from "@dfinity/agent";
import { HttpAgentOptions } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";

export const makeActorAzle = async (canisterId: string, createActor: any) => {
  return await createActor(canisterId, {
    agentOptions: {
      host: process.env.NEXT_PUBLIC_IC_HOST,
    },
  });
};

export function makeAzleActor() {
  return makeActorAzle(
    process.env.NEXT_PUBLIC_CANISTER_ID_AZLE_HELLO_WORLD!,
    azleActor
  );
}
