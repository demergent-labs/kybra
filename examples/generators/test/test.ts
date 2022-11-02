import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/generators/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/generators';

const generators_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code generators || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy generators`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(generators_canister as any)
];

run_tests(tests);
