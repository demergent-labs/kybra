import { runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/call_raw/test/tests';
import { createActor } from './dfx_generated/call_raw';

const callRawCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(callRawCanister as any));
