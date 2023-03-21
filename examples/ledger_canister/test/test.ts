import { getTests } from 'azle/examples/ledger_canister/test/tests';
import { createSnakeCaseProxy, runTests } from 'azle/test';
import { createActor } from './dfx_generated/ledger_canister';

const ledgerCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(ledgerCanister)));
