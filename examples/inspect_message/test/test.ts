import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/inspect_message/test/tests';
import { createActor } from './dfx_generated/inspect_message';

const inspectMessageCanister = createActor(getCanisterId('inspect_message'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(inspectMessageCanister)).filter((test) => {
        // TODO remove this once this is resolved: https://forum.dfinity.org/t/not-calling-accept-message-in-inspect-message-not-rejecting-immediately-in-dfx-0-14-2-beta-2/21105
        return (
            test.name !== 'not calling `ic.acceptMessage` in inspectMessage' &&
            test.name !== 'throwing in `inspectMessage`'
        );
    })
);
