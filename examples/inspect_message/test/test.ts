import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/inspect_message/test/tests';
import { createActor } from './dfx_generated/inspect_message';

const inspect_message_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(inspect_message_canister));
