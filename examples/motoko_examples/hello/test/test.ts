import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/hello/test/tests';
import { createActor } from './dfx_generated/hello';

const helloCanister = createActor(getCanisterId('hello'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(helloCanister)));
