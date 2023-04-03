import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/guard_functions/test/tests';
import { createActor } from './dfx_generated/guard_functions';

const functionGuardCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(functionGuardCanister)));
