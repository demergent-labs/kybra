import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/management_canister/test/tests';
import { createActor } from './dfx_generated/management_canister';

const managementCanister = createActor(getCanisterId('management_canister'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(managementCanister)));
