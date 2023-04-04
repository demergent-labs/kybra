from kybra import Async, blob, CallResult, ic, match, Record, update, Variant
from kybra.canisters.management import (
    management_canister,
    EcdsaPublicKeyResult,
    SignWithEcdsaResult,
)


class PublicKey(Record):
    public_key: blob


class Signature(Record):
    signature: blob


class PublicKeyResult(Variant, total=False):
    Ok: PublicKey
    Err: str


class SignResult(Variant, total=False):
    Ok: Signature
    Err: str


@update
def public_key() -> Async[PublicKeyResult]:
    caller = ic.caller().bytes
    call_result: CallResult[
        EcdsaPublicKeyResult
    ] = yield management_canister.ecdsa_public_key(
        {
            "canister_id": None,
            "derivation_path": [caller],
            "key_id": {"curve": {"secp256k1": None}, "name": "dfx_test_key"},
        }
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )


@update
def sign(message_hash: blob) -> Async[SignResult]:
    if len(message_hash) != 32:
        ic.trap("message_hash must be 32 bytes")

    caller = ic.caller().bytes

    call_result: CallResult[
        SignWithEcdsaResult
    ] = yield management_canister.sign_with_ecdsa(
        {
            "message_hash": message_hash,
            "derivation_path": [caller],
            "key_id": {"curve": {"secp256k1": None}, "name": "dfx_test_key"},
        }
    ).with_cycles(
        10_000_000_000
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
