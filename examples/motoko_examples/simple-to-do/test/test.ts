import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/simple-to-do/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/simple_to_do';

const simple_to_do_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code simple_to_do || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy simple_to_do`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(simple_to_do_canister as any)
];

run_tests(tests);
