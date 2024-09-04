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

        // TODO we can use the Azle test again once we update Azle to dfx 0.19.0
        // TODO Rejection code was changed from 3 to 5
        // TODO Update: This will probably just hang around until we update to jest
        if (test.name === 'update_query test') {
            return {
                ...test,
                test: async () => {
                    const result = await canister1.update_query();

                    return {
                        Ok:
                            'Err' in result &&
                            result.Err.includes(
                                `Rejection code 5, Error from Canister ${getCanisterId(
                                    'canister2'
                                )}: Canister has no query method`
                            )
                    };
                }
            };
        }

        return test;
    })
);
