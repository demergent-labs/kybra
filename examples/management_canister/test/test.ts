import { createSnakeCaseProxy, getCanisterId, ok, runTests } from 'azle/test';
import { getTests } from 'azle/examples/management_canister/test/tests';
import { createActor } from './dfx_generated/management_canister';

const managementCanister = createActor(getCanisterId('management_canister'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(managementCanister)).map((test) => {
        if (test.name === 'getCanisterStatus') {
            return {
                name: 'getCanisterStatus',
                test: async () => {
                    const canisterId =
                        await managementCanister.get_created_canister_id();

                    const getCanisterStatusResult =
                        await managementCanister.get_canister_status({
                            canister_id: canisterId
                        });

                    console.log(
                        'getCanisterStatusResult',
                        getCanisterStatusResult
                    );

                    if (!ok(getCanisterStatusResult)) {
                        return {
                            Err: getCanisterStatusResult.Err
                        };
                    }

                    const canisterStatus = getCanisterStatusResult.Ok;

                    return {
                        Ok:
                            'running' in canisterStatus.status &&
                            canisterStatus.memory_size === 342n &&
                            canisterStatus.cycles >= 800_000_000_000n &&
                            canisterStatus.settings.freezing_threshold ===
                                2_000_000n &&
                            canisterStatus.settings.controllers.length === 1 &&
                            canisterStatus.settings.memory_allocation ===
                                3_000_000n &&
                            canisterStatus.settings.compute_allocation === 1n &&
                            canisterStatus.module_hash.length === 0
                    };
                }
            };
        }

        return test;
    })
);
