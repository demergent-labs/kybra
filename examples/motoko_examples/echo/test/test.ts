import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/echo/test/tests';
import { createActor } from './dfx_generated/echo';

const echo_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(echo_canister));
