import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/management_canister/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/management_canister';

const management_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
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
                `dfx canister uninstall-code management_canister || true`,
                {
                    stdio: 'inherit'
                }
            );

            execSync(`dfx deploy management_canister`, {
                stdio: 'inherit'
            });

            execSync(
                `dfx ledger fabricate-cycles --canister management_canister --cycles 100000000000000`,
                {
                    stdio: 'inherit'
                }
            );
        }
    },
    ...get_tests(management_canister as any)
];

run_tests(tests);
