import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/persistent-storage/test/tests';
import { createActor } from './dfx_generated/persistent_storage';

const persistent_storage_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(persistent_storage_canister as any));
