import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/optional_types/test/tests';
import { createActor } from './dfx_generated/optional_types';

const optionalTypesCanister = createActor(getCanisterId('optional_types'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(optionalTypesCanister)));
