import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/complex_types/test/tests';
import { createActor } from './dfx_generated/complex_types';

const complex_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(complex_types_canister));
