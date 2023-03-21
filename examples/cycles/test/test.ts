// TODO If we want these tests to be more exact, we can check balances and make sure they are within some margin of error

import { getTests } from 'azle/examples/cycles/test/tests';
import { runTests } from 'azle/test';
import { createActor as createCyclesActor } from './dfx_generated/cycles';
import { createActor as createIntermediaryActor } from './dfx_generated/intermediary';

const cyclesCanister = createCyclesActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const intermediaryCanister = createIntermediaryActor(
    'ryjl3-tyaaa-aaaaa-aaaba-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

// TODO for now these tests need to be run on a fresh dfx start --clean, since cycles are not discarded on uninstall-code
runTests(getTests(cyclesCanister, intermediaryCanister));
