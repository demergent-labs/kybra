import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/threshold_ecdsa/test/tests';
import { createActor } from './dfx_generated/threshold_ecdsa';

const tecdsa_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(tecdsa_canister));
