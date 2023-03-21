import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/call_raw/test/tests';
import { createActor } from './dfx_generated/call_raw';

const call_raw_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(call_raw_canister as any));
