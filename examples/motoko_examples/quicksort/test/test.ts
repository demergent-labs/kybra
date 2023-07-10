import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/quicksort/test/tests';
import { createActor } from './dfx_generated/quicksort';

const quicksortCanister = createActor(getCanisterId('quicksort'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(quicksortCanister)));
