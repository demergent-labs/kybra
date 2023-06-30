import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/timers/test/tests';
import { createActor } from './dfx_generated/timers';

const timers_canister = createActor(getCanisterId('timers'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(timers_canister)));
