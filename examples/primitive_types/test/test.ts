import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/primitive_types/test/tests';
import { createActor } from './dfx_generated/primitive_types';

const primitive_types_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...get_tests(primitive_types_canister as any),
    // TODO once Azle has these tests, remove these tests and just use Azle's
    {
        name: 'get_text',
        test: async () => {
            const result = await primitive_types_canister.get_text();

            return {
                ok: result === 'this is a string defined with text'
            };
        }
    },
    {
        name: 'print_text',
        test: async () => {
            const result = await primitive_types_canister.print_text(
                'this is a string defined with text that is also printed'
            );

            return {
                ok:
                    result ===
                    'this is a string defined with text that is also printed'
            };
        }
    },
    {
        name: 'get_str',
        test: async () => {
            const result = await primitive_types_canister.get_str();

            return {
                ok: result === 'this is a string defined with str'
            };
        }
    },
    {
        name: 'print_str',
        test: async () => {
            const result = await primitive_types_canister.print_str(
                'this is a string defined with str that is also printed'
            );

            return {
                ok:
                    result ===
                    'this is a string defined with str that is also printed'
            };
        }
    },
    {
        name: 'get_bool',
        test: async () => {
            const result = await primitive_types_canister.get_bool();

            return {
                ok: result === true
            };
        }
    },
    {
        name: 'print_bool',
        test: async () => {
            const result = await primitive_types_canister.print_bool(false);

            return {
                ok: result === false
            };
        }
    }
];

run_tests(tests);
