import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/func_types/test/tests';
import { createActor } from './dfx_generated/func_types';

const funcTypesCanister = createActor(getCanisterId('func_types'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(funcTypesCanister)));
