// TODO Two of these tests are skipped because of this: https://forum.dfinity.org/t/embedding-wasm-dfx-0-17-0-crashing-where-previous-version-works/27916/6

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
            skip: true,
            test: async () => {
                execSync(
                    `dfx canister uninstall-code init_and_post_upgrade_recovery || true`,
                    {
                        stdio: 'inherit'
                    }
                );

                try {
                    execSync(
                        `dfx deploy init_and_post_upgrade_recovery --argument '(true)'`
                    );
                } catch (error: any) {
                    return {
                        Ok: error.stderr.toString().includes('init_ trapped')
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
            skip: true,
            test: async () => {
                try {
                    execSync(
                        `dfx deploy init_and_post_upgrade_recovery --argument '(true)'`
                    );
                } catch (error: any) {
                    return {
                        Ok: error.stderr
                            .toString()
                            .includes('post_upgrade_ trapped')
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
