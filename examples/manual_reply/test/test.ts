import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/manual_reply/test/tests';
import { createActor } from './dfx_generated/manual_reply';

const manual_reply_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code manual_reply || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy manual_reply`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(manual_reply_canister)
];

run_tests(tests);
