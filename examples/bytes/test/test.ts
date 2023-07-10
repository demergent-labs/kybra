import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/bytes/test/tests';
import { createActor } from './dfx_generated/bytes';

const bytesCanister = createActor(getCanisterId('bytes'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(bytesCanister)));
