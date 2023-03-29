import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/manual_reply/test/tests';
import { createActor } from './dfx_generated/manual_reply';

const manualReplyCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(manualReplyCanister)).filter(
        (test) => test.name !== 'update reply with inlineType'
    )
);
