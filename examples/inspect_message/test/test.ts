import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/inspect_message/test/tests';
import { createActor } from './dfx_generated/inspect_message';

const inspectMessageCanister = createActor(getCanisterId('inspect_message'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(inspectMessageCanister)));
