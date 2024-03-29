import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/query/test/tests';
import { createActor } from './dfx_generated/query';

const queryCanister = createActor(getCanisterId('query'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(queryCanister)));
