import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';
import { get_first_tests, get_second_tests } from './tests';

const stable_structures_canister_1 = createActorCanister1(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

const stable_structures_canister_2 = createActorCanister2(
    'ryjl3-tyaaa-aaaaa-aaaba-cai',
    { agentOptions: { host: 'http://127.0.0.1:8000' } }
);

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code canister1 || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx canister uninstall-code canister2 || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_first_tests(stable_structures_canister_1, 0, 6),
    ...get_first_tests(stable_structures_canister_2, 7, 13),
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx deploy`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_second_tests(stable_structures_canister_1, 0, 6),
    ...get_second_tests(stable_structures_canister_2, 7, 13)
];

run_tests(tests);
