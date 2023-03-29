import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/heartbeat/test/tests';
import { createActor as createActorHeartbeatAsync } from './dfx_generated/heartbeat_async';
import { createActor as createActorHeartbeatSync } from './dfx_generated/heartbeat_sync';

const heartbeatAsyncCanister = createActorHeartbeatAsync(
    'rrkah-fqaaa-aaaaa-aaaaq-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

const heartbeatSyncCanister = createActorHeartbeatSync(
    'ryjl3-tyaaa-aaaaa-aaaba-cai',
    {
        agentOptions: {
            host: 'http://127.0.0.1:8000'
        }
    }
);

runTests(
    getTests(
        createSnakeCaseProxy(heartbeatAsyncCanister),
        createSnakeCaseProxy(heartbeatSyncCanister)
    )
);
