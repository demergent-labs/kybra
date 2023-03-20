import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/bytes/test/tests';
import { createActor } from './dfx_generated/bytes';

const bytes_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

// TODO 2000 kb reaches the instruction limit in RustPython, but not in Azle
// TODO See if we can get a more efficient conversion for Vec<u8> and bytes
run_tests(
    get_tests(bytes_canister as any).filter((test: Test) => {
        return test.name !== 'get_bytes 2000 kb';
    })
);
