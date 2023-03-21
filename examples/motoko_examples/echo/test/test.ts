import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/echo/test/tests';
import { createActor } from './dfx_generated/echo';

const echoCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(echoCanister));
