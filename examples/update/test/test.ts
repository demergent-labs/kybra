import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/update';

const update_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code update || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy update`, {
                stdio: 'inherit'
            });
        }
    },
    {
        name: 'simple_update',
        test: async () => {
            const result = await update_canister.simple_update(
                'Why hello there'
            );

            return {
                ok: result === 'Why hello there'
            };
        }
    },
    {
        name: 'get_current_message',
        test: async () => {
            const result = await update_canister.get_current_message();

            return {
                ok: result === 'Why hello there'
            };
        }
    }
];

run_tests(tests);
