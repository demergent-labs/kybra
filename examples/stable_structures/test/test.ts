import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/stable_structures';
import { get_tests } from './tests';

const stable_structures_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code stable_structures || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy stable_structures`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(stable_structures_canister)
];

run_tests(tests);
