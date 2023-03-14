import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/audio_recorder/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/audio_recorder';

const audio_recorder_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code audio_recorder || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy audio_recorder`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(audio_recorder_canister)
];

run_tests(tests);
