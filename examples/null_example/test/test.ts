import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/null_example/test/tests';
import { createActor } from './dfx_generated/null_example';

const null_example_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(null_example_canister));
