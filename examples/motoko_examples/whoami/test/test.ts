import { runTests, Test } from 'azle/test';
import {
    callingIdentity,
    canisterId,
    getTests
} from 'azle/examples/motoko_examples/whoami/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/whoami';

const whoamiCanister = createActor(canisterId, {
    agentOptions: {
        host: 'http://127.0.0.1:8000',
        identity: callingIdentity
    }
});

const callingPrincipal = callingIdentity.getPrincipal().toString();

const tests: Test[] = [
    ...getTests(whoamiCanister as any).filter((test) => {
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
            const result = await whoamiCanister.argument();

            return {
                Ok: result.toString() === callingPrincipal
            };
        }
    }
];

runTests(tests);
