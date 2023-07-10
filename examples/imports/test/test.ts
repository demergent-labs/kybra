import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/imports/test/tests';
import { createActor } from './dfx_generated/imports';

const importsCanister = createActor(getCanisterId('imports'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests([
    ...getTests(createSnakeCaseProxy(importsCanister)),
    {
        name: 'boltons_floor',
        test: async () => {
            const result = await importsCanister.boltons_floor(456.76);

            return {
                Ok: result === 456n
            };
        }
    }
]);
