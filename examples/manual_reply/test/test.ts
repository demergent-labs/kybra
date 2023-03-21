import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/manual_reply/test/tests';
import { createActor } from './dfx_generated/manual_reply';

const manual_reply_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(manual_reply_canister as any));
