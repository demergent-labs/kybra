import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/imports/test/tests';
import { createActor } from './dfx_generated/imports';

const imports_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...get_tests(imports_canister as any),
    {
        name: 'get_math_message',
        test: async () => {
            const result = await imports_canister.get_math_message();

            return {
                ok: result === 11n
            };
        }
    }
    // TODO add a test like this back in once we have a pip installable package to test on
    // {
    //     name: 'get_external_module_message',
    //     test: async () => {
    //         const result = await imports_canister.get_external_module_message();

    //         return {
    //             ok: result === 3n
    //         };
    //     }
    // }
];

run_tests(tests);
