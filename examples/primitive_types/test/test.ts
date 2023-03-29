import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/primitive_types/test/tests';
import { createActor } from './dfx_generated/primitive_types';

const primitiveTypesCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(primitiveTypesCanister)).filter(
        (value) => value.name != 'getNumber' && value.name != 'printNumber'
    )
);
