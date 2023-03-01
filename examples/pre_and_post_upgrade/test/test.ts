import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/pre_and_post_upgrade/test/tests';
import { createActor } from './dfx_generated/pre_and_post_upgrade';

const pre_and_post_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(pre_and_post_canister));
