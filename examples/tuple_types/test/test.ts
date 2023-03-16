import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/tuple_types/test/tests';
import { createActor } from './dfx_generated/tuple_types';

const tuple_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(
    get_tests(tuple_types_canister as any).filter(
        (test) => test.name !== 'two_tuple_with_inline_records' // Kybra does not have the concept of inline records
    )
);
