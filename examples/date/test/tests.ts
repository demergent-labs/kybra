import { Test } from 'azle/test';
import { _SERVICE } from './dfx_generated/date/date.did.d';
import { ActorSubclass } from '@dfinity/agent';

export function getTests(dateCanister: ActorSubclass<_SERVICE>): Test[] {
    return [
        {
            name: 'test_get_date',
            test: async () => {
                const result = await dateCanister.get_date(
                    '2023-06-26T00:00:00.000'
                );
                return {
                    Ok: result === 26n
                };
            }
        },
        {
            name: 'test_get_day',
            test: async () => {
                const result = await dateCanister.get_day(
                    '2023-06-26T00:00:00.000'
                );
                return {
                    Ok: result === 0n
                };
            }
        },
        {
            name: 'test_get_full_year',
            test: async () => {
                const result = await dateCanister.get_full_year(
                    '2023-06-26T00:00:00.000'
                );
                return {
                    Ok: result === 2023n
                };
            }
        },
        {
            name: 'test_get_hours',
            test: async () => {
                const result = await dateCanister.get_hours(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 12n
                };
            }
        },
        {
            name: 'test_get_milliseconds',
            test: async () => {
                const result = await dateCanister.get_milliseconds(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 789n
                };
            }
        },
        {
            name: 'test_get_minutes',
            test: async () => {
                const result = await dateCanister.get_minutes(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 34n
                };
            }
        },
        {
            name: 'test_get_month',
            test: async () => {
                const result = await dateCanister.get_month(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 6n
                };
            }
        },
        {
            name: 'test_get_seconds',
            test: async () => {
                const result = await dateCanister.get_seconds(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 56n
                };
            }
        },
        {
            name: 'test_get_time',
            test: async () => {
                const result = await dateCanister.get_time(
                    '1970-01-01T00:00:01.000'
                );
                return {
                    Ok: result === 1000n
                };
            }
        },
        {
            name: 'test_get_timezone_offset',
            test: async () => {
                // Assuming the server is set to UTC
                const result = await dateCanister.get_timezone_offset(
                    '2023-06-26T12:34:56.789'
                );
                return {
                    Ok: result === 0n
                };
            }
        }
    ];
}
