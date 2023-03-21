import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/phone-book/test/tests';
import { createActor } from './dfx_generated/phone_book';

const phoneBookCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(phoneBookCanister as any));
