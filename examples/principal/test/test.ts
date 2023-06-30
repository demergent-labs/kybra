import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/principal/test/tests';
import { createActor } from './dfx_generated/principal';

const principalCanister = createActor(getCanisterId('principal'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(principalCanister)));
