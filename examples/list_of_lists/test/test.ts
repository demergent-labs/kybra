import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { getTests } from 'azle/examples/list_of_lists/test/tests';
import { createActor } from './dfx_generated/list_of_lists';

const listOfListsCanister = createActor(getCanisterId('list_of_lists'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(listOfListsCanister)));
