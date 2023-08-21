import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'getMessage' : ActorMethod<[], string>,
  'setMessage' : ActorMethod<[string], undefined>,
}
