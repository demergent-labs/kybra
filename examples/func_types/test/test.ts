import { runTests } from 'azle/test';
import { getTests } from 'azle/examples/func_types/test/tests';
import { createActor } from './dfx_generated/func_types';

const funcTypesCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(funcTypesCanister as any));
