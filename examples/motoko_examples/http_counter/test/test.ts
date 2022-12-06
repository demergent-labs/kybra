import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/http_counter/test/tests';
import { execSync } from 'child_process';

const tests: Test[] = [
    // {
    //     name: 'init get count',
    //     test: async () => {
    //         console.log(getCount());
    //         console.log(getExpectedCountResult(0));
    //         return {
    //             ok: getCount() === getExpectedGetCountResult(0)
    //         };
    //     }
    // },
    // {
    //     name: 'first increment',
    //     test: async () => {
    //         return {
    //             ok: count() === getExpectedCountResult(1)
    //         };
    //     }
    // },
    // {
    //     name: 'second increment',
    //     test: async () => {
    //         return {
    //             ok: count() === getExpectedCountResult(2)
    //         };
    //     }
    // },
    // {
    //     name: 'get count',
    //     test: async () => {
    //         return {
    //             ok: getCount() === getExpectedGetCountResult(2)
    //         };
    //     }
    // },
    // {
    //     name: 'gzipped increment',
    //     test: async () => {
    //         return {
    //             ok: countGzip() === 'update'
    //         };
    //     }
    // },
    // {
    //     name: 'get gzipped count',
    //     test: async () => {
    //         return {
    //             ok: getCountGzip() === 'query'
    //         };
    //     }
    // },
    // {
    //     name: 'get streaming count',
    //     test: async () => {
    //         return {
    //             ok: getCountStream() === getExpectedGetCountStreamResult(3)
    //         };
    //     }
    // },
    // {
    //     name: 'final get count',
    //     test: async () => {
    //         return {
    //             ok: getCount() === getExpectedGetCountResult(3)
    //         };
    //     }
    // }
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code http_counter || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy http_counter`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests()
];

run_tests(tests);

function getCanisterID(): string {
    return execSync(`dfx canister id http_counter`).toString().trim();
}

function count(): string {
    const canister_id = getCanisterID();
    return execSync(
        `curl --silent -X POST "${canister_id}.localhost:8000/" --resolve "${canister_id}.localhost:8000:127.0.0.1"`
    )
        .toString()
        .trim();
}

function countGzip(): string {
    const canister_id = getCanisterID();
    return execSync(
        `curl --compressed --silent -X POST "${canister_id}.localhost:8000/" --resolve "${canister_id}.localhost:8000:127.0.0.1"`
    )
        .toString()
        .trim();
}

function getCount(): string {
    const canister_id = getCanisterID();
    return execSync(
        `curl --silent "${canister_id}.localhost:8000/" --resolve "${canister_id}.localhost:8000:127.0.0.1"`
    )
        .toString()
        .trim();
}

function getCountStream(): string {
    const canister_id = getCanisterID();
    return execSync(
        `curl --silent "${canister_id}.localhost:8000/stream" --resolve "${canister_id}.localhost:8000:127.0.0.1"`
    )
        .toString()
        .trim();
}

function getCountGzip(): string {
    const canister_id = getCanisterID();
    return execSync(
        `curl --compressed --silent "${canister_id}.localhost:8000/" --resolve "${canister_id}.localhost:8000:127.0.0.1"`
    )
        .toString()
        .trim();
}

function getExpectedGetCountResult(expectedCount: number): string {
    return `Counter is ${expectedCount}\n/`;
}

function getExpectedGetCountStreamResult(expectedCount: number): string {
    return `Counter is ${expectedCount} streaming`;
}

function getExpectedCountResult(expectedCount: number): string {
    return `Counter updated to ${expectedCount}`;
}
