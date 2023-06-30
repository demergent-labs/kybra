import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/manual_reply/test/tests';
import { createActor } from './dfx_generated/manual_reply';

const manualReplyCanister = createActor(getCanisterId('manual_reply'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(manualReplyCanister)).filter((test) => {
        return (
            test.name !== 'update reply with void' &&
            test.name !== 'query reply with void'
        ); // TODO remove this once this issue is addressed: https://github.com/demergent-labs/kybra/issues/416
    })
);
