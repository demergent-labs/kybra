import { getCanisterId, runTests } from 'azle/test';
import { createActor } from './dfx_generated/stdlib';
import { getTests } from './tests';

const stdlibCanister = createActor(getCanisterId('stdlib'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(stdlibCanister));
