import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/simple_erc20/test/tests';
import { createActor } from './dfx_generated/simple_erc20';

const simpleErc20Canister = createActor(getCanisterId('simple_erc20'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(simpleErc20Canister)));
