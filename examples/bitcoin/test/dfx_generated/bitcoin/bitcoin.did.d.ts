import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ExecuteGetBalanceResult = { ok: bigint } | { err: string };
export type ExecuteGetCurrentFeePercentiles =
    | { ok: BigUint64Array }
    | { err: string };
export type ExecuteGetUtxosResult = { ok: GetUtxosResult } | { err: string };
export type ExecuteSendTransactionResult = { ok: null } | { err: string };
export interface GetUtxosResult {
    next_page: [] | [Uint8Array];
    tip_height: number;
    tip_block_hash: Uint8Array;
    utxos: Array<Utxo>;
}
export interface Outpoint {
    txid: Uint8Array;
    vout: number;
}
export interface Utxo {
    height: number;
    value: bigint;
    outpoint: Outpoint;
}
export interface _SERVICE {
    get_balance: ActorMethod<[string], ExecuteGetBalanceResult>;
    get_current_fee_percentiles: ActorMethod<
        [],
        ExecuteGetCurrentFeePercentiles
    >;
    get_utxos: ActorMethod<[string], ExecuteGetUtxosResult>;
    send_transaction: ActorMethod<[Uint8Array], ExecuteSendTransactionResult>;
}
