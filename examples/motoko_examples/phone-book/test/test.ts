import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/phone-book/test/tests';
import { createActor } from './dfx_generated/phone_book';

const phone_book_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(phone_book_canister as any));
