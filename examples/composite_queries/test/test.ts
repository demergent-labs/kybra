import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/composite_queries/test/tests';
import { createActor } from './dfx_generated/canister1';

const canister1 = createActor(getCanisterId('canister1'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(canister1)).map((test) => {
        if (test.name === 'inc_canister1 test') {
            return {
                name: 'inc_canister1 test',
                test: async () => {
                    const result = await canister1.inc_canister1();

                    return {
                        Ok: 'Ok' in result && result.Ok === 3n
                    };
                }
            };
        }

        return test;
    })
);
