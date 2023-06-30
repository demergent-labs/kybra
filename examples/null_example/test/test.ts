import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/null_example/test/tests';
import { createActor } from './dfx_generated/null_example';

const nullExampleCanister = createActor(getCanisterId('null_example'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(nullExampleCanister)));
