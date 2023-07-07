import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/threshold_ecdsa/test/tests';
import { createActor } from './dfx_generated/threshold_ecdsa';

const tecdsaCanister = createActor(getCanisterId('threshold_ecdsa'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(tecdsaCanister)));
