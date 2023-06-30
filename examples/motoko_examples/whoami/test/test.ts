import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import {
    callingIdentity,
    getTests
} from 'azle/examples/motoko_examples/whoami/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/whoami';

const whoamiCanister = createActor(getCanisterId('whoami'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000',
        identity: callingIdentity
    }
});

runTests(
    getTests(createSnakeCaseProxy(whoamiCanister), 'whoami').map((test) => {
        if (test.name === 'installer') {
            return {
                name: 'installer',
                test: async () => {
                    const result = await whoamiCanister.installer();

                    return {
                        Ok:
                            result.toString() ===
                            execSync(`dfx canister id whoami`).toString().trim()
                    };
                }
            };
        }

        return test;
    })
);
