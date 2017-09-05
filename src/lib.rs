extern crate ring;
extern crate serde;

#[macro_use]
extern crate serde_derive;

// use serde::ser::Serialize;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

type PubKey = [u8; 32];
type HashResult = u64;
type KeyPair = [u8; 85];
type SignatureVec = Vec<u8>;
type Address = HashResult;

const genesis_amount: u64 = 2099;


#[derive(Serialize, Deserialize, Debug)]
enum Message {
    QueryBlock(u64),
    TransactionMsg(Transaction),
    BlockMsg(Block)
}
#[derive(Serialize, Deserialize, Debug)]
struct TransactionHeader {
    amount: u64,
    from: PubKey,
    to: Address,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    hdr: TransactionHeader,
    sig: SignatureVec,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockHeader {
    txs: Vec<Transaction>,
    miner: PubKey,
    prevblk: HashResult,
    nonce: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    idx: u64,
    hdr: BlockHeader,
    sig: SignatureVec,
}

#[derive(Debug)]
pub struct Node {
    blockchain: Vec<Block>,
    ledger: HashMap<HashResult, u64>,
    txbuf: Vec<Transaction>,
    keypair: KeyPair,
}

fn generate_genesis() -> Block {
    let creator_pub: &'static [u8; 32] = b"\xc9\xebqp\xb8\xb3\xa74\t\xdc\x95=\xa6\x81\xd7\x95\xd0a'N\x8ft\x94Ad\x90\roAA\xbaa";
    let prev = 0x6d797468;

    let mut hasher = DefaultHasher::new();
    hasher.write(creator_pub);
    let hashh = hasher.finish();

    let txhdr = TransactionHeader {
        amount: genesis_amount,
        from: creator_pub.clone(),
        to: hashh,
    };

    let tx = Transaction {
        hdr: txhdr,
        sig: vec![188, 121, 66, 18, 173, 23, 81, 59, 53, 29, 33, 179, 114, 182, 140, 55, 237, 38, 146, 236, 207, 227, 75, 234, 103, 236, 167, 183, 253, 168, 203, 31, 112, 181, 128, 222, 97, 73, 139, 42, 170, 96, 194, 56, 121, 207, 186, 255, 180, 96, 212, 155, 113, 21, 12, 24, 62, 112, 81, 88, 194, 20, 143, 6]
    };

    let header = BlockHeader {
        txs: vec![tx],
        miner: creator_pub.clone(),
        prevblk: prev,
        nonce: 0
    };

    Block {
        idx: 0,
        hdr: header,
        sig: vec![112, 16, 150, 33, 135, 82, 16, 152, 158, 236, 126, 6, 229, 237, 194, 37, 44, 213, 54, 204, 211, 93, 43, 244, 138, 178, 225, 45, 36, 141, 18, 193, 16, 162, 211, 106, 110, 228, 133, 33, 240, 125, 250, 50, 252, 177, 214, 175, 139, 215, 155, 114, 7, 211, 229, 233, 218, 39, 9, 103, 27, 88, 164, 6],
    }
}


impl Node {
    pub fn new() -> Node {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        let genesis = generate_genesis();

        let mut ledger = HashMap::new();
        ledger.insert(genesis.hdr.txs.first().unwrap().hdr.to, genesis_amount);

        println!("yay got the node");

        Node {
            blockchain: vec![genesis],
            ledger: ledger,
            txbuf: Vec::new(),
            keypair: pkcs8_bytes,
        }

    }

    pub fn run(&self) {
        println!("running");
    }
}

#[cfg(test)]

mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    extern crate ring;
    extern crate untrusted;

    // #[test]
    fn wrf() {
        let mut hashr = DefaultHasher::new();
        hashr.write(b"huhh");
        let x = hashr.finish();
        println!("sddd {:?}", x);
    }


    #[test]
    fn azzz() {
        let x = ring::signature::Ed25519KeyPair.generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();

        let kp = ring::signature::Ed25519KeyPair.from_pkcs8(
            untrusted::Input::from(x)).unwrap();


        let y = x.public_key_bytes();
        println!("yyy {:?}", y);
    }
}
