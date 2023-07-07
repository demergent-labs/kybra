import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/randomness/test/tests';
import { createActor } from './dfx_generated/randomness';

const randomnessCanister = createActor(getCanisterId('randomness'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(randomnessCanister)));
