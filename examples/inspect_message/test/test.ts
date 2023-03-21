import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/inspect_message/test/tests';
import { createActor } from './dfx_generated/inspect_message';

const inspectMessageCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(inspectMessageCanister));
