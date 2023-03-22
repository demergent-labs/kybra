import { createSnakeCaseProxy, runTests } from 'azle/test';
import { get_tests as getTests } from 'azle/examples/cross_canister_calls/test/tests';
import { createActor as createActorCanister1 } from './dfx_generated/canister1';
import { createActor as createActorCanister2 } from './dfx_generated/canister2';

const canister1 = createActorCanister1('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const canister2 = createActorCanister2('ryjl3-tyaaa-aaaaa-aaaba-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    get_tests(createSnakeCaseProxy(canister1), createSnakeCaseProxy(canister2))
);

import { ok, Test } from 'azle/test';
import { _SERVICE as CANISTER1_SERVICE } from './dfx_generated/canister1/canister1.did';
import { _SERVICE as CANISTER2_SERVICE } from './dfx_generated/canister2/canister2.did';
import { ActorSubclass } from '@dfinity/agent';

export function get_tests(
    canister1: ActorSubclass<CANISTER1_SERVICE>,
    canister2: ActorSubclass<CANISTER2_SERVICE>
): Test[] {
    return [
        {
            name: 'canister1 balance 0',
            test: async () => {
                const result = await canister1.balance('0');

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok === 100n
                };
            }
        },
        {
            name: 'canister1 account 0',
            test: async () => {
                const result = await canister1.account({
                    id: '0'
                });

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok:
                        result.Ok.length === 1 &&
                        result.Ok[0].id === '0' &&
                        result.Ok[0].balance === 100n
                };
            }
        },
        {
            name: 'canister1 balance 1',
            test: async () => {
                const result = await canister1.balance('1');

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok === 0n
                };
            }
        },
        {
            name: 'canister1 account 1',
            test: async () => {
                const result = await canister1.account({
                    id: '1'
                });

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok.length === 0
                };
            }
        },
        {
            name: 'canister1 accounts',
            test: async () => {
                const result = await canister1.accounts();

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok:
                        result.Ok.length === 1 &&
                        result.Ok[0].id === '0' &&
                        result.Ok[0].balance === 100n
                };
            }
        },
        {
            name: 'canister1 transfer',
            test: async () => {
                const result = await canister1.transfer('0', '1', 34n);

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok === 34n
                };
            }
        },
        {
            name: 'canister1 balance 0',
            test: async () => {
                const result = await canister1.balance('0');

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok === 66n
                };
            }
        },
        {
            name: 'canister1 account 0',
            test: async () => {
                const result = await canister1.account({
                    id: '0'
                });

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok:
                        result.Ok.length === 1 &&
                        result.Ok[0].id === '0' &&
                        result.Ok[0].balance === 66n
                };
            }
        },
        {
            name: 'canister1 balance 1',
            test: async () => {
                const result = await canister1.balance('1');

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok: result.Ok === 34n
                };
            }
        },
        {
            name: 'canister1 account 1',
            test: async () => {
                const result = await canister1.account({
                    id: '1'
                });

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok:
                        result.Ok.length === 1 &&
                        result.Ok[0].id === '1' &&
                        result.Ok[0].balance === 34n
                };
            }
        },
        {
            name: 'canister1 accounts',
            test: async () => {
                const result = await canister1.accounts();

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                return {
                    Ok:
                        result.Ok.length === 2 &&
                        result.Ok[0].id === '0' &&
                        result.Ok[0].balance === 66n &&
                        result.Ok[1].id === '1' &&
                        result.Ok[1].balance === 34n
                };
            }
        },
        {
            name: 'canister1 trap',
            test: async () => {
                const result = await canister1.trap();

                return {
                    Ok:
                        'Err' in result &&
                        result.Err ===
                            'Rejection code 5, IC0503: Canister ryjl3-tyaaa-aaaaa-aaaba-cai trapped explicitly: hahahaha'
                };
            }
        },
        {
            name: 'canister2 get_notification empty',
            test: async () => {
                const result = await canister2.getNotification();

                return {
                    Ok: result === ''
                };
            }
        },
        {
            name: 'canister1 send_notification',
            test: async () => {
                console.log('We are at the beginning');
                const result = await canister1.sendNotification();

                console.log(result);

                return {
                    Ok: 'Ok' in result && result.Ok === null
                };
            }
        },
        {
            name: 'canister2 get_notification',
            test: async () => {
                const result = await canister2.getNotification();

                return {
                    Ok: result === 'This is the notification'
                };
            }
        }
    ];
}
