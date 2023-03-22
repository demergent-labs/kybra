import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/outgoing_http_requests/test/tests';
import { createActor } from './dfx_generated/outgoing_http_requests';

const outgoingHttpRequestsCanister = createActor(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(getTests(createSnakeCaseProxy(outgoingHttpRequestsCanister)));
