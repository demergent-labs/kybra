import { ok, runTests, Test } from 'azle/test';
import {
    impure_setup as impureSetup,
    while_running_bitcoin_daemon as whileRunningBitcoinDaemon
} from 'azle/examples/bitcoin/test/setup';
import { createActor } from './dfx_generated/bitcoin';
import { wallets } from 'azle/examples/bitcoin/test/wallets';
import { State } from 'azle/examples/bitcoin/test/test';
import { bitcoin_cli } from 'azle/examples/bitcoin/test/bitcoin_cli';

const bitcoinCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const state: State = {
    signed_tx_hex: ''
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
                    return { err: result.err };
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
                    return { err: result.err };
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
                    return { err: result.err };
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
                    bitcoin_cli.get_received_by_address(wallets.bob.p2wpkh);

                const tx_bytes = hex_string_to_bytes(state.signed_tx_hex);

                const result = await bitcoinCanister.send_transaction(
                    Array.from(tx_bytes)
                );

                if (!ok(result)) {
                    return {
                        err: result.err
                    };
                }

                bitcoin_cli.generate_to_address(1, wallets.alice.p2wpkh);

                // Wait for generated block to be pulled into replica
                await new Promise((resolve) => setTimeout(resolve, 5000));

                const balance_after_transaction =
                    bitcoin_cli.get_received_by_address(wallets.bob.p2wpkh, 0);

                return {
                    Ok:
                        result.Ok === null &&
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
