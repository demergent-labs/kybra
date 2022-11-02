import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/pre_and_post_upgrade/test/tests';
import { createActor } from './dfx_generated/pre_and_post_upgrade';

const pre_and_post_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(
                `dfx canister uninstall-code pre_and_post_upgrade || true`,
                {
                    stdio: 'inherit'
                }
            );

            execSync(`dfx deploy pre_and_post_upgrade`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(pre_and_post_canister)
];

run_tests(tests);
