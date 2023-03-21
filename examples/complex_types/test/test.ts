import { createSnakeCaseProxy, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/complex_types/test/tests';
import { createActor } from './dfx_generated/complex_types';

const complexTypesCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(complexTypesCanister)));
