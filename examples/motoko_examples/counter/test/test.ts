import { getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/counter/test/tests';
import { createActor } from './dfx_generated/counter';

const counterCanister = createActor(getCanisterId('counter'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(counterCanister as any));
