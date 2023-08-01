import { getCanisterId, runTests } from 'azle/test';
import { createActor } from '../../src/declarations/backend/index.js';
import { get_tests } from './tests';
import { HttpAgent } from "@dfinity/agent";


let canisterId = getCanisterId('backend');

const backend = createActor(canisterId,{
    agent: new HttpAgent({
        host: "http://localhost:4943",
    }),
});

runTests(get_tests(backend));