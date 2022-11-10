import { execSync } from 'child_process';
import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/func_types/test/tests';
import { createActor } from './dfx_generated/func_types';

const func_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy func_types',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code func_types || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy func_types`, {
                stdio: 'inherit'
            });
        }
    },
    {
        name: 'deploy notifiers',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code notifiers || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy notifiers`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(func_types_canister)
];

run_tests(tests);
