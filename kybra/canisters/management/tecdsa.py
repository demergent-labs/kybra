from kybra import blob, null, Opt, Principal, Record, Variant, Vec


class EcdsaCurve(Variant):
    secp256k1: null


class KeyId(Record):
    curve: EcdsaCurve
    name: str


class EcdsaPublicKeyArgs(Record):
    canister_id: Opt[Principal]
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
