import { createSnakeCaseProxy, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/cross_canister_calls/test/tests';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';

const canister1 = createActorCanister1('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const canister2 = createActorCanister2('ryjl3-tyaaa-aaaaa-aaaba-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(canister1), createSnakeCaseProxy(canister2))
);
