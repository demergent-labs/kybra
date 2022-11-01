import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { get_tests } from 'azle/examples/init/test/tests';
import { createActor } from './dfx_generated/init';

const init_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...deploy(
        'init',
        `'(record { id = "0" }, variant { Fire }, principal "rrkah-fqaaa-aaaaa-aaaaq-cai")'`
    ),
    ...get_tests(init_canister)
];

run_tests(tests);

export function deploy(canister_name: string, argument?: string): Test[] {
    return [
        {
            // TODO hopefully we can get rid of this: https://forum.dfinity.org/t/generated-declarations-in-node-js-environment-break/12686/16?u=lastmjs
            name: 'waiting for createActor fetchRootKey',
            wait: 5000
        },
        {
            name: `create canister ${canister_name}`,
            prep: async () => {
                execSync(`dfx canister create ${canister_name}`, {
                    stdio: 'inherit'
                });
            }
        },
        {
            name: 'clear canister memory',
            prep: async () => {
                execSync(
                    `dfx canister uninstall-code ${canister_name} || true`,
                    {
                        stdio: 'inherit'
                    }
                );
            }
        },
        {
            name: `build canister ${canister_name}`,
            prep: async () => {
                execSync(`dfx build ${canister_name}`, {
                    stdio: 'inherit'
                });
            }
        },
        {
            name: `install canister ${canister_name}`,
            prep: async () => {
                execSync(
                    `dfx canister install${
                        argument === undefined ? '' : ` --argument ${argument}`
                    } ${canister_name} --wasm .dfx/kybra/${canister_name}/target/wasm32-unknown-unknown/release/${canister_name}.wasm.gz`,
                    {
                        stdio: 'inherit'
                    }
                );
            }
        }
    ];
}
