import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/guard_functions/test/tests';
import { createActor } from './dfx_generated/guard_functions';
import { AgentError } from '@dfinity/agent/lib/cjs/errors';

const functionGuardCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

let tests: Test[] = [
    ...getTests(createSnakeCaseProxy(functionGuardCanister)).filter((value) => {
        return (
            value.name !== 'looselyGuardedWithGuardOptionKeyAsString' &&
            value.name !== 'invalidReturnTypeGuarded' &&
            value.name !== 'badObjectGuarded' &&
            value.name !== 'nonNullOkValueGuarded' &&
            value.name !== 'nonStringErrValueGuarded'
        );
    }),
    // TODO we should make these errors more presentable
    {
        name: 'invalid_return_type_guarded',
        test: async () => {
            try {
                await functionGuardCanister.invalid_return_type_guarded();
                return {
                    Err: 'invalid_return_type_guarded should have had an error'
                };
            } catch (err) {
                return {
                    Ok: (err as AgentError).message.includes(
                        'TypeError: Expected NoneType but received str'
                    )
                };
            }
        }
    },
    {
        name: 'bad_object_guarded',
        test: async () => {
            try {
                await functionGuardCanister.bad_object_guarded();
                return { Err: 'bad_object_guarded should have had an error' };
            } catch (err) {
                return {
                    Ok: (err as AgentError).message.includes(
                        'NameError(\\"name \'badProp\' is not defined\\")'
                    )
                };
            }
        }
    },
    {
        name: 'non_null_ok_value_guarded',
        test: async () => {
            try {
                await functionGuardCanister.non_null_ok_value_guarded();
                return {
                    Err: 'non_null_ok_value_guarded should have had an error'
                };
            } catch (err) {
                return {
                    Ok: (err as AgentError).message.includes(
                        'NameError(\\"name \'Ok\' is not defined\\")'
                    )
                };
            }
        }
    },
    {
        name: 'non_string_err_value_guarded',
        test: async () => {
            try {
                await functionGuardCanister.non_string_err_value_guarded();
                return {
                    Err: 'non_string_err_value_guarded should have had an error'
                };
            } catch (err) {
                return {
                    Ok: (err as AgentError).message.includes(
                        'NameError(\\"name \'Err\' is not defined\\")'
                    )
                };
            }
        }
    }
];

runTests(tests);
