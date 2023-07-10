import { ActorSubclass } from '@dfinity/agent';
import { Test } from 'azle/test';
import { execSync } from 'child_process';
import { _SERVICE } from './dfx_generated/init_and_post_upgrade_recovery/init_and_post_upgrade_recovery.did';

export function getTests(
    initAndPostUpgradeRecoveryCanister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'init should fail',
            test: async () => {
                execSync(
                    `dfx canister uninstall-code init_and_post_upgrade_recovery || true`,
                    {
                        stdio: 'inherit'
                    }
                );

                execSync(
                    `dfx deploy init_and_post_upgrade_recovery --argument '(true)' || true`,
                    {
                        stdio: 'inherit'
                    }
                );

                try {
                    await initAndPostUpgradeRecoveryCanister.get_message();
                } catch (error) {
                    return {
                        Ok: (error as any)
                            .toString()
                            .includes('SystemError: missing python interpreter')
                    };
                }

                return {
                    Ok: false
                };
            }
        },
        {
            name: 'init should succeed',
            test: async () => {
                execSync(
                    `dfx deploy init_and_post_upgrade_recovery --argument '(false)'`,
                    {
                        stdio: 'inherit'
                    }
                );

                const messageResult =
                    await initAndPostUpgradeRecoveryCanister.get_message();

                return {
                    Ok: messageResult === 'init_'
                };
            }
        },
        {
            name: 'post_upgrade should fail',
            test: async () => {
                execSync(
                    `dfx deploy init_and_post_upgrade_recovery --argument '(true)' || true`,
                    {
                        stdio: 'inherit'
                    }
                );

                try {
                    await initAndPostUpgradeRecoveryCanister.get_message();
                } catch (error) {
                    console.log(error);

                    return {
                        Ok: (error as any)
                            .toString()
                            .includes('SystemError: missing python interpreter')
                    };
                }

                return {
                    Ok: false
                };
            }
        },
        {
            name: 'post_upgrade should succeed',
            test: async () => {
                execSync(
                    `dfx deploy init_and_post_upgrade_recovery --argument '(false)'`,
                    {
                        stdio: 'inherit'
                    }
                );

                const messageResult =
                    await initAndPostUpgradeRecoveryCanister.get_message();

                return {
                    Ok: messageResult === 'post_upgrade_'
                };
            }
        }
    ];
}
