import { getCanisterId, ok, runTests, Test } from 'azle/test';
import {
    impureSetup,
    whileRunningBitcoinDaemon
} from 'azle/examples/bitcoin/test/setup';
import { createActor } from './dfx_generated/bitcoin';
import { wallets } from 'azle/examples/bitcoin/test/wallets';
import { State } from 'azle/examples/bitcoin/test/test';
import { bitcoinCli } from 'azle/examples/bitcoin/test/bitcoin_cli';

const bitcoinCanister = createActor(getCanisterId('bitcoin'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const state: State = {
    signedTxHex: ''
};

const tests: Test[] = [
    ...impureSetup(wallets, state),
    {
        name: 'wait for blockchain balance to reflect',
        wait: 30_000
    },
    ...testCanisterFunctionality()
];

whileRunningBitcoinDaemon(() => runTests(tests));

function testCanisterFunctionality() {
    return [
        {
            name: 'get_balance',
            test: async () => {
                const result = await bitcoinCanister.get_balance(
                    wallets.alice.p2wpkh
                );

                if (!ok(result)) {
                    return { Err: result.Err };
                }

                const block_reward = 5_000_000_000n;
                const blocks_mined_in_setup = 101n;
                const expected_balance = block_reward * blocks_mined_in_setup;

                return {
                    Ok: result.Ok === expected_balance
                };
            }
        },
        {
            name: 'get_utxos',
            test: async () => {
                const result = await bitcoinCanister.get_utxos(
                    wallets.alice.p2wpkh
                );

                if (!ok(result)) {
                    return { Err: result.Err };
                }

                return {
                    Ok:
                        result.Ok.tip_height === 101 &&
                        result.Ok.utxos.length === 101
                };
            }
        },
        {
            name: 'get_current_fee_percentiles',
            test: async () => {
                const result =
                    await bitcoinCanister.get_current_fee_percentiles();

                if (!ok(result)) {
                    return { Err: result.Err };
                }

                return {
                    Ok: result.Ok.length === 0 // TODO: This should have entries
                };
            }
        },
        {
            name: 'send transaction',
            test: async () => {
                const balance_before_transaction =
                    bitcoinCli.getReceivedByAddress(wallets.bob.p2wpkh);

                const tx_bytes = hex_string_to_bytes(state.signedTxHex);

                const result = await bitcoinCanister.send_transaction(tx_bytes);

                if (!ok(result)) {
                    return {
                        Err: result.Err
                    };
                }

                bitcoinCli.generateToAddress(1, wallets.alice.p2wpkh);

                // Wait for generated block to be pulled into replica
                await new Promise((resolve) => setTimeout(resolve, 5000));

                const balance_after_transaction =
                    bitcoinCli.getReceivedByAddress(wallets.bob.p2wpkh, 0);

                return {
                    Ok:
                        result.Ok === true &&
                        balance_before_transaction === 0 &&
                        balance_after_transaction === 1
                };
            }
        }
    ];
}

/**
 * Converts a hex string into an array of bytes
 * @param hex The hex string to convert
 * @returns The data as bytes
 */
function hex_string_to_bytes(hex: string): Uint8Array {
    return Uint8Array.from(
        hex.match(/.{1,2}/g)?.map((byte) => parseInt(byte, 16)) || []
    );
}
