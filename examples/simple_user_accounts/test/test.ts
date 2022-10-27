import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/simple_user_accounts/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/simple_user_accounts';

const simple_user_accounts_canister = createActor(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(
                `dfx canister uninstall-code simple_user_accounts || true`,
                {
                    stdio: 'inherit'
                }
            );

            execSync(`dfx deploy simple_user_accounts`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(simple_user_accounts_canister)
];

run_tests(tests);
