import { createSnakeCaseProxy, runTests } from 'azle/test';
import {
    callingIdentity,
    canisterId,
    getTests
} from 'azle/examples/motoko_examples/whoami/test/tests';
import { createActor } from './dfx_generated/whoami';

const whoamiCanister = createActor(canisterId, {
    agentOptions: {
        host: 'http://127.0.0.1:8000',
        identity: callingIdentity
    }
});

runTests(getTests(createSnakeCaseProxy(whoamiCanister), 'whoami'));
