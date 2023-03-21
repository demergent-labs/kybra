import { run_tests } from 'azle/test';
import { get_tests } from 'azle/examples/outgoing_http_requests/test/tests';
import { createActor } from './dfx_generated/outgoing_http_requests';

const outgoing_http_requests_canister = createActor(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

run_tests(get_tests(outgoing_http_requests_canister as any));
