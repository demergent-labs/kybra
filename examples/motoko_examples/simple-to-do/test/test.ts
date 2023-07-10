import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/simple-to-do/test/tests';
import { createActor } from './dfx_generated/simple_to_do';

const simpleToDoCanister = createActor(getCanisterId('simple_to_do'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(simpleToDoCanister)));
