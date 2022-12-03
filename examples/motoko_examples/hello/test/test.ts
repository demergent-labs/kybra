import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/hello/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/hello';

const hello_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code hello || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy hello`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(hello_canister)
];

run_tests(tests);
