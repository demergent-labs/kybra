from kybra import blob, null, opt, Principal, Record, Variant, Vec


class EcdsaCurve(Variant):
    secp256k1: null


class KeyId(Record):
    curve: EcdsaCurve
    name: str


class EcdsaPublicKeyArgs(Record):
    canister_id: opt[Principal]
    derivation_path: Vec[blob]
    key_id: KeyId


class SignWithEcdsaArgs(Record):
    message_hash: blob
    derivation_path: Vec[blob]
    key_id: KeyId


class EcdsaPublicKeyResult(Record):
    public_key: blob
    chain_code: blob


class SignWithEcdsaResult(Record):
    signature: blob
