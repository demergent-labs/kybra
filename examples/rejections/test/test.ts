import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/rejections/test/tests';
import { createActor } from './dfx_generated/rejections';

const rejections_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(rejections_canister as any));
