import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/query/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/query';

const query_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code query || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy query`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(query_canister)
];

run_tests(tests);
