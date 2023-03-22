import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/primitive_types/test/tests';
import { createActor } from './dfx_generated/primitive_types';

const primitiveTypesCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...getTests(createSnakeCaseProxy(primitiveTypesCanister)),
    // TODO once Azle has these tests, remove these tests and just use Azle's
    {
        name: 'get_text',
        test: async () => {
            const result = await primitiveTypesCanister.get_text();

            return {
                Ok: result === 'this is a string defined with text'
            };
        }
    },
    {
        name: 'print_text',
        test: async () => {
            const result = await primitiveTypesCanister.print_text(
                'this is a string defined with text that is also printed'
            );

            return {
                Ok:
                    result ===
                    'this is a string defined with text that is also printed'
            };
        }
    },
    {
        name: 'get_str',
        test: async () => {
            const result = await primitiveTypesCanister.get_str();

            return {
                Ok: result === 'this is a string defined with str'
            };
        }
    },
    {
        name: 'print_str',
        test: async () => {
            const result = await primitiveTypesCanister.print_str(
                'this is a string defined with str that is also printed'
            );

            return {
                Ok:
                    result ===
                    'this is a string defined with str that is also printed'
            };
        }
    },
    {
        name: 'get_bool',
        test: async () => {
            const result = await primitiveTypesCanister.get_bool();

            return {
                Ok: result === true
            };
        }
    },
    {
        name: 'print_bool',
        test: async () => {
            const result = await primitiveTypesCanister.print_bool(false);

            return {
                Ok: result === false
            };
        }
    }
];

runTests(tests);
