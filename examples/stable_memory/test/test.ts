import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/stable_memory/test/tests';
import { createActor } from './dfx_generated/stable_memory';

const stable_memory_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

run_tests(
    get_tests(stable_memory_canister as any).filter(
        (test) => test.name !== 'stable64 grow out of memory'
    )
);
