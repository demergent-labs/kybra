import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/complex_init/test/tests';
import { createActor } from './dfx_generated/complex_init';

const complex_init_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(complex_init_canister));
