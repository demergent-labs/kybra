import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/blob_array/test/tests';
import { createActor } from './dfx_generated/blob_array';

const blobArrayCanister = createActor(getCanisterId('blob_array'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(blobArrayCanister)));
