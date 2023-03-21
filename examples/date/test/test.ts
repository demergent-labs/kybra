import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/date/test/tests';
import { createActor } from './dfx_generated/date';

const dateCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...getTests(createSnakeCaseProxy(dateCanister)),
    {
        name: 'get_time',
        test: async () => {
            const result = await dateCanister.get_time();

            const seconds = new Date().getTime() / 1_000;

            return {
                Ok: seconds - result < 10
            };
        }
    },
    {
        name: 'get_strftime',
        test: async () => {
            const result = await dateCanister.get_strftime();

            const now = new Date().toDateString();

            return {
                Ok: result === now
            };
        }
    }
];

runTests(tests);
