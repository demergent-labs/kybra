import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/tuple_types/test/tests';
import { createActor } from './dfx_generated/tuple_types';

const tuple_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(tuple_types_canister)).filter(
        (test) => test.name !== 'twoTupleWithInlineRecords' // Kybra does not have the concept of inline records
    )
);
