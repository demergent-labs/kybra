import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/minimal-counter-dapp/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/minimal_dapp';

const minimal_counter_dapp_canister = createActor(
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

            execSync(`dfx canister uninstall-code minimal_dapp || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy minimal_dapp`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(minimal_counter_dapp_canister as any)
];

run_tests(tests);
