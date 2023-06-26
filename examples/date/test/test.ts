import { runTests, Test } from 'azle/test';
import { createActor } from './dfx_generated/date';
import { getTests } from './tests';

const dateCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = getTests(dateCanister);

runTests(tests);
