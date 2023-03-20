import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/simple_erc20/test/tests';
import { createActor } from './dfx_generated/simple_erc20';

const simple_erc20_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(simple_erc20_canister));
