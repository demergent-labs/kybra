import { getCanisterId } from 'azle/dfx';
import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/guard_functions/test/tests';
import { createActor } from './dfx_generated/guard_functions';
import { AgentError } from '@dfinity/agent/lib/cjs/errors';

const functionGuardCanister = createActor(getCanisterId('guard_functions'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

let tests: Test[] = [
    ...getTests(createSnakeCaseProxy(functionGuardCanister)).filter((value) => {
        return (
            value.name !== 'callExpressionWithEmptyOptionsObject' &&
            value.name !== 'looselyGuardedWithGuardOptionKeyAsString' &&
            value.name !== 'invalidReturnTypeGuarded' &&
            value.name !== 'badObjectGuarded' &&
            value.name !== 'nonNullOkValueGuarded' &&
            value.name !== 'nonStringErrValueGuarded' &&
            value.name !== 'modifyStateGuarded' &&
            value.name !== 'tightlyGuarded' &&
            value.name !== 'errorStringGuarded' &&
            value.name !== 'customErrorGuarded'
        );
    }),
    {
        name: 'customErrorGuarded',
        test: async () => {
            try {
                const result =
                    await functionGuardCanister.custom_error_guarded();
                return {
                    Err: 'Expected customErrorGuarded function to throw'
                };
            } catch (error) {
                return {
                    Ok: (error as AgentError).message.includes(
                        `Execution halted by \\"throw custom error\\" guard function`
                    )
                };
            }
        }
    },
    {
        name: 'errorStringGuarded',
        test: async () => {
            try {
                const result =
                    await functionGuardCanister.error_string_guarded();
                return {
                    Err: 'Expected errorStringGuarded function to throw'
                };
            } catch (error) {
                return {
                    Ok: (error as AgentError).message.includes(
                        `Execution halted by \\"throw string\\" guard function`
                    )
                };
            }
        }
    },
    {
        name: 'tightlyGuarded',
        test: async () => {
            try {
                const result = await functionGuardCanister.tightly_guarded();
                return {
                    Err: 'Expected tightlyGuarded function to throw'
                };
            } catch (error) {
                return {
                    Ok: (error as AgentError).message.includes(
                        `Execution halted by \\"unpassable\\" guard function`
                    )
                };
            }
        }
    },
    {
        name: 'modifyStateGuarded',
        test: async () => {
            const counterBefore = (await functionGuardCanister.get_state())
                .counter;
            const methodExecuted =
                await functionGuardCanister.modify_state_guarded();
            const counterAfter = (await functionGuardCanister.get_state())
                .counter;

            return {
                Ok: counterBefore === 0 && methodExecuted && counterAfter === 1
            };
        }
    },
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
                        'TypeError: expected Result but received str'
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
                        'TypeError: expected Result but received dict'
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
                        'TypeError: expected NoneType but received str'
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
                        "TypeError: Expected type 'str' but 'dict' found"
                    )
                };
            }
        }
    },
    {
        name: 'name_error_guarded',
        test: async () => {
            try {
                await functionGuardCanister.name_error_guarded();
                return {
                    Err: 'name_error_guarded should have had an error'
                };
            } catch (err) {
                return {
                    Ok: (err as AgentError).message.includes(
                        "NameError: name 'Ok' is not defined"
                    )
                };
            }
        }
    }
];

runTests(tests);
