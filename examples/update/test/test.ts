import { run_tests, Test } from 'azle/test';
import { createActor } from './dfx_generated/update';

const update_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
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
