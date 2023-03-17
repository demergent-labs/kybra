import { run_tests } from 'azle/test';
import { get_tests } from './tests';
import { createActor } from './dfx_generated/function_guards';

const function_guard_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(function_guard_canister));
