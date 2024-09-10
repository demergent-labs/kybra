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

                    if (!ok(getCanisterStatusResult)) {
                        return {
                            Err: getCanisterStatusResult.Err
                        };
                    }

                    const canisterStatus = getCanisterStatusResult.Ok;

                    return {
                        Ok:
                            'running' in canisterStatus.status &&
                            canisterStatus.memory_size === 366n &&
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

        if (test.name === 'executeDepositCycles') {
            return {
                name: 'executeDepositCycles',
                test: async () => {
                    const canisterId =
                        await managementCanister.get_created_canister_id();

                    const statusBeforeResult =
                        await managementCanister.get_canister_status({
                            canister_id: canisterId
                        });

                    if (!ok(statusBeforeResult)) {
                        return {
                            Err: statusBeforeResult.Err
                        };
                    }

                    const statusBefore = statusBeforeResult.Ok;
                    const cyclesBefore = statusBefore.cycles;

                    const depositCyclesResult =
                        await managementCanister.execute_deposit_cycles(
                            canisterId
                        );

                    if (!ok(depositCyclesResult)) {
                        return {
                            Err: depositCyclesResult.Err
                        };
                    }

                    const statusAfterResult =
                        await managementCanister.get_canister_status({
                            canister_id: canisterId
                        });

                    if (!ok(statusAfterResult)) {
                        return {
                            Err: statusAfterResult.Err
                        };
                    }

                    const statusAfter = statusAfterResult.Ok;
                    const cyclesAfter = statusAfter.cycles;

                    return {
                        Ok: cyclesAfter > cyclesBefore
                    };
                }
            };
        }

        return test;
    })
);
