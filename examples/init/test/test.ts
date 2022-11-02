import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/init/test/tests';
import { createActor } from './dfx_generated/init';

const init_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code init || true`, {
                stdio: 'inherit'
            });

            execSync(
                `dfx deploy --argument '(record { id = "0" }, variant { Fire }, principal "rrkah-fqaaa-aaaaa-aaaaq-cai")' init`,
                {
                    stdio: 'inherit'
                }
            );
        }
    },
    ...get_tests(init_canister)
];

run_tests(tests);
