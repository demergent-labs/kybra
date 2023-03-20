import { run_tests, Test } from 'azle/test';
import { createActor } from './dfx_generated/date';

const date_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'get_time',
        test: async () => {
            const result = await date_canister.get_time();

            const seconds = new Date().getTime() / 1_000;

            return {
                ok: seconds - result < 10
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
