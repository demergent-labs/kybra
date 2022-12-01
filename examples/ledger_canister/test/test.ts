import { get_tests } from 'azle/examples/ledger_canister/test/tests';
import { run_tests, Test } from 'azle/test';
import { createActor } from './dfx_generated/ledger_canister';

const ledger_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [...get_tests(ledger_canister as any, 'src/icp_ledger')];

run_tests(tests);
