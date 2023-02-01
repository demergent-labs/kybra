import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/func_types/test/tests';
import { createActor } from './dfx_generated/func_types';

const func_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(func_types_canister as any));
