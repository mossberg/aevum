extern crate ring;

use std::collections::HashMap;

type PubKey = [u8; 32];
type HashResult = u64;
type KeyPair = [u8; 85];

struct Transaction {
    amount: u64,
    from: PubKey,
    to: u64,
    sig: ring::signature::Signature,
}

struct Block {
    txs: Vec<Transaction>,
    minerpub: PubKey,
    hash: HashResult,
    nonce: u64,
    sig: ring::signature::Signature
}

pub struct Node {
    blockchain: Vec<Block>,
    ledger: HashMap<HashResult, u64>,
    txbuf: Vec<Transaction>,
    keypair: KeyPair,
}

impl Node {
    pub fn new() -> Node {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

        Node {
            blockchain: Vec::new(),
            ledger: HashMap::new(),
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
    use ring;

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
