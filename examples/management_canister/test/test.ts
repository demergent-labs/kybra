import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/management_canister/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/management_canister';

const management_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [...get_tests(management_canister)];

run_tests(tests);
