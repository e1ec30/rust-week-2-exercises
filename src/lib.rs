use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    match decode(hex_str) {
        Err(_e) => Err("hex decode error".into()),
        Ok(v) => Ok(v),
    }
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>
    let v: Vec<u8> = bytes.iter().copied().rev().collect();
    v
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    match input.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Invalid satoshi amount".into()),
    }
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    match script[0] {
        0x00 => ScriptType::P2WPKH, // e1ec30: Starts with OP_0
        0x76 if script[1] == 0xa9 => ScriptType::P2PKH, //e1ec30: Starts with OP_DUP, OP_HASH160
        _ => ScriptType::Unknown,
    }
}

// TODO: complete Outpoint tuple struct
// e1ec30: An Outpoint is a txid:vout pair, where vout is the index of the output in the transaction referenced by txid.
#[derive(Debug)]
pub struct Outpoint(pub String, pub u64);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    *balance -= fee;
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {txid}")
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0x76 => Ok(Self::OpDup),
            0xac => Ok(Self::OpChecksig),
            _ => Err(format!("Invalid opcode: 0x{byte:02x}")),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    utxo
}
