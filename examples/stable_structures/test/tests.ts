import { Test } from 'azle/test';
import { _SERVICE } from './dfx_generated/stable_structures/stable_structures.did';
import { ActorSubclass } from '@dfinity/agent';

export function get_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [];
}
