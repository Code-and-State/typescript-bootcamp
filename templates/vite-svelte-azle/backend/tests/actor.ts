import { Actor, HttpAgent } from '@dfinity/agent';
import { getCanisterId } from 'azle/test';
import { idlFactory } from "./declarations/backend/backend.did";
import type { IDL } from '@dfinity/candid';
import type { Principal } from '@dfinity/principal';
import type { ActorSubclass } from '@dfinity/agent';

function createActor<T>(
  canisterId: string | Principal,
  idlFactory: IDL.InterfaceFactory,
): ActorSubclass<T> {
  const agent = new HttpAgent({
    host: 'http://localhost:4943',
  });
    agent.fetchRootKey().catch((err) => {
      console.warn(
        'Unable to fetch root key. Check to ensure that your local replica is running',
      );
      console.error(err);
    });
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
}

let canisterId = getCanisterId('backend');

export const backend = createActor(canisterId, idlFactory);
