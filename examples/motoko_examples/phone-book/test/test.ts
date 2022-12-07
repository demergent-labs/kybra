import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/phone-book/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/phone_book';

const phone_book_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code phone_book || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy phone_book`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(phone_book_canister as any)
];

run_tests(tests);
