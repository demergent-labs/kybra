import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/query/test/tests';
import { createActor } from './dfx_generated/query';

const query_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(query_canister));
