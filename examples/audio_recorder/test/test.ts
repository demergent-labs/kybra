import { createSnakeCaseProxy, getCanisterId, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/audio_recorder/test/tests';
import { createActor } from './dfx_generated/audio_recorder';

const audioRecorderCanister = createActor(getCanisterId('audio_recorder'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(audioRecorderCanister)));
