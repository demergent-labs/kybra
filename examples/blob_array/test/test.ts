import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/blob_array/test/tests';
import { createActor } from './dfx_generated/blob_array';

const blob_array_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(blob_array_canister as any));
