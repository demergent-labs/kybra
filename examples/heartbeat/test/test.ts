import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/heartbeat/test/tests';
import { createActor } from './dfx_generated/heartbeat';

const heartbeatCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(heartbeatCanister));
