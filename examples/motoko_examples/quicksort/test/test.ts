import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/quicksort/test/tests';
import { createActor } from './dfx_generated/quicksort';

const quicksort_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(quicksort_canister as any));
