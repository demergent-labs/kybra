import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/simple-to-do/test/tests';
import { createActor } from './dfx_generated/simple_to_do';

const simple_to_do_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(simple_to_do_canister as any));
