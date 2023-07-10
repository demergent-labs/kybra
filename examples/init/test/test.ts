import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/init/test/tests';
import { createActor } from './dfx_generated/init';

const initCanister = createActor(getCanisterId('init'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(initCanister)));
