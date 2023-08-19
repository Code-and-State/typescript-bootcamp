import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'add' : ActorMethod<[bigint], bigint>,
  'get' : ActorMethod<[], bigint>,
  'inc' : ActorMethod<[], bigint>,
}
