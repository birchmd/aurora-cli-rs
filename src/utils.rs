use aurora_engine_transactions::eip_1559::{SignedTransaction1559, Transaction1559, TYPE_BYTE};
use aurora_engine_types::{types::Address, U256};
use rlp::RlpStream;
use secp256k1::{Message, PublicKey, SecretKey};

pub(crate) fn hex_to_arr32(h: &str) -> Result<[u8; 32], hex::FromHexError> {
    let mut output = [0u8; 32];
    hex::decode_to_slice(h, &mut output)?;
    Ok(output)
}

pub(crate) fn address_from_secret_key(sk: &SecretKey) -> Address {
    let pk = PublicKey::from_secret_key(sk);
    let hash = aurora_engine_sdk::keccak(&pk.serialize()[1..]);
    Address::try_from_slice(&hash[12..]).unwrap()
}

pub(crate) fn sign_transaction(
    tx: Transaction1559,
    secret_key: &SecretKey,
) -> SignedTransaction1559 {
    let mut rlp_stream = RlpStream::new();
    rlp_stream.append(&TYPE_BYTE);
    tx.rlp_append_unsigned(&mut rlp_stream);
    let message_hash = aurora_engine_sdk::keccak(rlp_stream.as_raw());
    let message = Message::parse_slice(message_hash.as_bytes()).unwrap();

    let (signature, recovery_id) = secp256k1::sign(&message, secret_key);
    let r = U256::from_big_endian(&signature.r.b32());
    let s = U256::from_big_endian(&signature.s.b32());
    SignedTransaction1559 {
        transaction: tx,
        parity: recovery_id.serialize(),
        r,
        s,
    }
}
