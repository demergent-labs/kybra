import { createSnakeCaseProxy, runTests } from 'azle/test';
import { get_tests as getTests } from './tests';
import { createActor } from './dfx_generated/guard_functions';

const functionGuardCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(functionGuardCanister));
