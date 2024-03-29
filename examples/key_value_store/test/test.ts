import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/key_value_store/test/tests';
import { createActor } from './dfx_generated/key_value_store';

const keyValueStoreCanister = createActor(getCanisterId('key_value_store'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(keyValueStoreCanister)));
