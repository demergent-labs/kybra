import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/cross_canister_calls/test/tests';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';

const canister1 = createActorCanister1(getCanisterId('canister1'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const canister2 = createActorCanister2(getCanisterId('canister2'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(
        createSnakeCaseProxy(canister1),
        createSnakeCaseProxy(canister2)
    ).map((test) => {
        if (test.name === 'canister1 trap') {
            return {
                ...test,
                test: async () => {
                    const result = await canister1.trap();
                    const expected = `Rejection code 5, IC0503: Error from Canister ${getCanisterId(
                        'canister2'
                    )}: Canister called \`ic0.trap\` with message: hahahaha.`;

                    return {
                        Ok: 'Err' in result && result.Err.includes(expected)
                    };
                }
            };
        }
        return test;
    })
);
