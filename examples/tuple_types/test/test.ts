import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/tuple_types/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/tuple_types';

const tuple_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code tuple_types || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy tuple_types`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(tuple_types_canister).filter(
        (test) => test.name !== 'two_tuple_with_inline_records' // Kybra does not have the concept of inline records
    )
];

run_tests(tests);
