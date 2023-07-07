import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/phone-book/test/tests';
import { createActor } from './dfx_generated/phone_book';

const phoneBookCanister = createActor(getCanisterId('phone_book'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(phoneBookCanister)));
