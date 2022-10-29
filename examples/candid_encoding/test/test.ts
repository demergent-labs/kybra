import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/candid_encoding/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/candid_encoding';

const candid_encoding_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code candid_encoding || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy candid_encoding`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(candid_encoding_canister as any)
];

run_tests(tests);
