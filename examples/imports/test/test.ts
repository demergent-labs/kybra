import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/imports/test/tests';
import { createActor } from './dfx_generated/imports';

const importsCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...getTests(createSnakeCaseProxy(importsCanister)),
    {
        name: 'get_math_message',
        test: async () => {
            const result = await importsCanister.get_math_message();

            return {
                Ok: result === 11n
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

runTests(tests);
