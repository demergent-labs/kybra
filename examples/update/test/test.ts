import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/update/test/tests';
import { createActor } from './dfx_generated/update';

const updateCanister = createActor(getCanisterId('update'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(updateCanister)));
