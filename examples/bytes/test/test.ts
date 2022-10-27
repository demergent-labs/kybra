import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/bytes/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/bytes';

const bytes_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code bytes || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy bytes`, {
                stdio: 'inherit'
            });
        }
    },
    // TODO 2000 kb reaches the instruction limit in RustPython, but not in Azle
    // TODO See if we can get a more efficient conversion for Vec<u8> and bytes
    ...get_tests(bytes_canister as any).filter((test) => {
        return test.name !== 'get_bytes 2000 kb';
    })
];

run_tests(tests);
