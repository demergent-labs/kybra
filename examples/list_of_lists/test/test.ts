import { Principal } from '@dfinity/principal';
import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/list_of_lists/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/list_of_lists';
import { deepEqual } from 'fast-equals';
import { Opt } from 'azle';

const list_of_lists_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code list_of_lists || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy list_of_lists`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(list_of_lists_canister as any)
];

run_tests(tests);
