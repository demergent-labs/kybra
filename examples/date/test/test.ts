import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/date';

const date_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code date || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy date`, {
                stdio: 'inherit'
            });
        }
    },
    {
        name: 'get_time',
        test: async () => {
            const result = await date_canister.get_time();

            const seconds = new Date().getTime() / 1_000;

            return {
                ok: seconds - result < 5
            };
        }
    },
    {
        name: 'get_strftime',
        test: async () => {
            const result = await date_canister.get_strftime();

            const now = new Date().toDateString();

            return {
                ok: result === now
            };
        }
    }
];

run_tests(tests);
