import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/simple-to-do/test/tests';
import { createActor } from './dfx_generated/simple_to_do';

const simpleToDoCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(simpleToDoCanister as any));
