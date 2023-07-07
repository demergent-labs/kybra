import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/candid_encoding/test/tests';
import { createActor } from './dfx_generated/candid_encoding';

const candidEncodingCanister = createActor(getCanisterId('candid_encoding'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(candidEncodingCanister)));
