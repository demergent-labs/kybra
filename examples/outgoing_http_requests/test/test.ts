import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/outgoing_http_requests/test/tests';
import { createActor } from './dfx_generated/outgoing_http_requests';

const outgoingHttpRequestsCanister = createActor(
    getCanisterId('outgoing_http_requests'),
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(getTests(createSnakeCaseProxy(outgoingHttpRequestsCanister)));
