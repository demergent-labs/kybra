import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/persistent-storage/test/tests';
import { createActor } from './dfx_generated/persistent_storage';

const persistentStorageCanister = createActor(
    getCanisterId('persistent_storage'),
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(getTests(createSnakeCaseProxy(persistentStorageCanister)));
