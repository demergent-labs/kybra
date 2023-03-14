import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/hello-world/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/hello_world';

const hello_world_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(get_tests(hello_world_canister));
