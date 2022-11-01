import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/heartbeat/test/tests';
import { createActor } from './dfx_generated/heartbeat';

const heartbeat_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code heartbeat || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy heartbeat`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(heartbeat_canister)
];

run_tests(tests);
