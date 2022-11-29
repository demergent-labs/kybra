import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/stable_storage/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/stable_storage';

const stable_storage_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code stable_storage || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy stable_storage`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(stable_storage_canister as any)
];

run_tests(tests);
