import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/candid_encoding/test/tests';
import { createActor } from './dfx_generated/candid_encoding';

const candid_encoding_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(candid_encoding_canister as any));
