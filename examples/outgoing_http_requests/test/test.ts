import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/outgoing_http_requests/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/outgoing_http_requests';

const outgoing_http_requests_canister = createActor(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(
                `dfx canister uninstall-code outgoing_http_requests || true`,
                {
                    stdio: 'inherit'
                }
            );

            execSync(`dfx deploy outgoing_http_requests`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(outgoing_http_requests_canister as any)
];

run_tests(tests);
