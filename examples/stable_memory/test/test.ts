import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/stable_memory/test/tests';
import { createActor } from './dfx_generated/stable_memory';

const stable_memory_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(stable_memory_canister)));
