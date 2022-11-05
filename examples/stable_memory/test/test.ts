import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/stable_memory/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/stable_memory';

const stable_memory_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code stable_memory || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy stable_memory`, {
                stdio: 'inherit'
            });
        }
    },
    // TODO As of dfx 0.12.0, stable memory is much larger
    // TODO We need to redo this test in Azle and Kybra with that in mind
    ...get_tests(stable_memory_canister as any).filter(
        (test) => test.name !== 'stable64 grow out of memory'
    )
];

run_tests(tests);
