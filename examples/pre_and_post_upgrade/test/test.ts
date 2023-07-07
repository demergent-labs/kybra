import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/pre_and_post_upgrade/test/tests';
import { createActor } from './dfx_generated/pre_and_post_upgrade';

const preAndPostCanister = createActor(getCanisterId('pre_and_post_upgrade'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(preAndPostCanister)));
