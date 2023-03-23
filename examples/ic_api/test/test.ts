import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/ic_api/test/tests';
import { createActor } from './dfx_generated/ic_api';

const icApiCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(icApiCanister)));
