import { run_tests, Test } from 'azle/test';
import {
    callingIdentity,
    canisterId,
    get_tests,
    someonePrincipal
} from 'azle/examples/motoko_examples/whoami/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/whoami';

const whoami_canister = createActor(canisterId, {
    agentOptions: {
        host: 'http://127.0.0.1:8000',
        identity: callingIdentity
    }
});

const callingPrincipal = callingIdentity.getPrincipal().toString();

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code whoami || true`, {
                stdio: 'inherit'
            });

            execSync(
                `dfx deploy --argument '(principal "${someonePrincipal}")' whoami`,
                {
                    stdio: 'inherit'
                }
            );
        }
    },
    ...get_tests(whoami_canister as any).filter((test) => {
        return test.name !== 'redeploy' && test.name !== 'updated argument';
    }),
    {
        name: 'redeploy',
        prep: async () => {
            execSync(
                `dfx deploy --argument '(principal "${callingPrincipal}")'`,
                {
                    stdio: 'inherit'
                }
            );
        }
    },
    {
        name: 'updated argument',
        test: async () => {
            const result = await whoami_canister.argument();

            return {
                ok: result.toString() === callingPrincipal
            };
        }
    }
];

run_tests(tests);
