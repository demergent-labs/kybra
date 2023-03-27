import { createSnakeCaseProxy, runTests, Test } from 'azle/test';
import { execSync } from 'child_process';
import {
    preRedeployTests,
    postRedeployTests,
    insertErrorTests
} from 'azle/examples/stable_structures/test/tests';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';
import { createActor as createActorCanister3 } from './dfx_generated/canister3';

const stableStructuresCanister1 = createActorCanister1(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    { agentOptions: { host: 'http://127.0.0.1:8000' } }
);

const stableStructuresCanister2 = createActorCanister2(
    'ryjl3-tyaaa-aaaaa-aaaba-cai',
    { agentOptions: { host: 'http://127.0.0.1:8000' } }
);

const stableStructuresCanister3 = createActorCanister3(
    'r7inp-6aaaa-aaaaa-aaabq-cai',
    { agentOptions: { host: 'http://127.0.0.1:8000' } }
);

const tests: Test[] = [
    ...preRedeployTests(createSnakeCaseProxy(stableStructuresCanister1), 0, 4),
    ...preRedeployTests(createSnakeCaseProxy(stableStructuresCanister2), 5, 9),
    ...preRedeployTests(
        createSnakeCaseProxy(stableStructuresCanister3),
        10,
        13
    ),
    {
        name: 'redeploy canisters',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx deploy`, {
                stdio: 'inherit'
            });
        }
    },
    ...postRedeployTests(createSnakeCaseProxy(stableStructuresCanister1), 0, 4),
    ...postRedeployTests(createSnakeCaseProxy(stableStructuresCanister2), 5, 9),
    ...postRedeployTests(
        createSnakeCaseProxy(stableStructuresCanister3),
        10,
        13
    ),
    ...insertErrorTests(
        createSnakeCaseProxy(stableStructuresCanister1),
        createSnakeCaseProxy(stableStructuresCanister3)
    )
];

runTests(tests);
