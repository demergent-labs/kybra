import { getCanisterId, runTests } from 'azle/test';
import { getTests } from './tests';
import { createActor } from './dfx_generated/init_and_post_upgrade_recovery';

const initAndPostUpgradeRecoveryCanister = createActor(
    getCanisterId('init_and_post_upgrade_recovery'),
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(getTests(initAndPostUpgradeRecoveryCanister));
