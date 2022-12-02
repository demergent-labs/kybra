import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/ethereum_json_rpc/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/ethereum_json_rpc';

if (process.env.ETHEREUM_URL === undefined) {
    throw new Error(
        `No Ethereum URL set, did you set the ETHEREUM_URL environment variable?\nExample: ETHEREUM_URL=https://some-ethereum-provider-url.org npm test`
    );
}

const ethereum_json_rpc_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code ethereum_json_rpc || true`, {
                stdio: 'inherit'
            });

            execSync(
                `dfx deploy --argument '("${process.env.ETHEREUM_URL}")' ethereum_json_rpc`,
                {
                    stdio: 'inherit'
                }
            );
        }
    },
    ...get_tests(ethereum_json_rpc_canister as any)
];

run_tests(tests);
