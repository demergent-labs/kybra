import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/key_value_store/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/key_value_store';

const key_value_store_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code key_value_store || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy key_value_store`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(key_value_store_canister)
];

run_tests(tests);
