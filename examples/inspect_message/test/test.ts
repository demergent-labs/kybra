import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/inspect_message/test/tests';
import { createActor } from './dfx_generated/inspect_message';

const inspect_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code inspect_message || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy inspect_message`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(inspect_canister)
];

run_tests(tests);
