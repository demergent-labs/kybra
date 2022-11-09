import { get_tests } from 'azle/examples/ledger_canister/test/tests';
import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/ledger_canister';

const ledger_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    // TODO need to do my own deployment because directory structure is different in Kybra
    // {
    //     name: 'deploy',
    //     prep: async () => {
    //         await new Promise((resolve) => setTimeout(resolve, 5000));

    //         execSync(`dfx canister uninstall-code ledger_canister || true`, {
    //             stdio: 'inherit'
    //         });

    //         execSync(`dfx deploy ledger_canister`, {
    //             stdio: 'inherit'
    //         });

    //         execSync(
    //             `dfx ledger fabricate-cycles --canister ledger_canister --cycles 100000000000000`,
    //             {
    //                 stdio: 'inherit'
    //             }
    //         );
    //     }
    // },
    ...get_tests(ledger_canister)
];

run_tests(tests);
