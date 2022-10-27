import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/optional_types/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/optional_types';

const optional_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code optional_types || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy optional_types`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(optional_types_canister)
];

run_tests(tests);
