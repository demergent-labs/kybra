import { getTests } from 'azle/examples/ledger_canister/test/tests';
import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { createActor } from './dfx_generated/ledger_canister';

const ledgerCanister = createActor(getCanisterId('ledger_canister'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(ledgerCanister)));
