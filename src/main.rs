use eth_encode_packed::SolidityDataType;
use eth_encode_packed::abi;
use eth_encode_packed::ethabi::ethereum_types::{U256};
use sha3::{Digest, Keccak256};

fn main () {
    let mut buf = [0u8; 64];
    let mut nonce: u128 = 0;
    let mut sequence = 1;
    let token_id = 1;
    let mut target: &[u8] = &[105, 115, 0];
    loop {
        let input = vec![
            SolidityDataType::Number(U256::from(token_id)),
            SolidityDataType::Number(U256::from(sequence)),
            SolidityDataType::Number(U256::from(nonce)),
        ];
        let (bytes, _hash) = abi::encode_packed(&input);
        let mut hasher = Keccak256::new();
        hasher.update(bytes);
        let hash = hasher.finalize();
        if hash.starts_with(target) {
            println!(
                "{} {} {} {}",
                token_id,
                sequence,
                nonce,
                base16ct::lower::encode_str(&hash, &mut buf).unwrap()
            );
            nonce = 0;
            sequence += 1;
            target = if sequence % 2 == 0 {
                &[110, 111, 116]
            } else {
                &[105, 115, 0]
            }
        } else {
            nonce += 1;
        }
    }
}
