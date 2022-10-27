import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/simple_erc20/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/simple_erc20';

const simple_erc20_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code simple_erc20 || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy simple_erc20`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(simple_erc20_canister)
];

run_tests(tests);
