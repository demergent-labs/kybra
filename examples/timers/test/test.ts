import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/timers/test/tests';
import { createActor } from './dfx_generated/timers';

const timers_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(timers_canister));
