import { ActorSubclass } from '@dfinity/agent';
import { Test } from 'azle/test';
import { _SERVICE } from '../../src/declarations/backend/backend.did.d';

export function get_tests(backend: ActorSubclass<_SERVICE>): Test[] {
  return [
    {
      name: 'backend value 0',
      test: async () => {
        const result = await backend.get();

        return {
          Ok: result == 0n,
        };
      },
    },
    {
      name: 'backend inc 1',
      test: async () => {
        const result = await backend.inc();

        return {
          Ok: result == 1n,
        };
      },
    },
    {
      name: 'count add 5',
      test: async () => {
        const result = await backend.add(BigInt(5));
        return {
          Ok: result === 6n,
        };
      },
    },
  ];
}
