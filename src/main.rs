use prost::Message;

pub mod pegged {
    include!("./binding/protos/pegged.rs");
}

fn main() {
    // Mint
    let mut mint_request = pegged::Mint::default();
    mint_request.token = b"PHA".to_vec();
    mint_request.account = b"Alice".to_vec();
    mint_request.amount = b"10000".to_vec();
    mint_request.depositor = b"Bob".to_vec();
    mint_request.ref_chain_id = 1;
    mint_request.ref_id = b"ref id".to_vec();

    let mut buf = Vec::new();
    buf.reserve(mint_request.encoded_len());
    mint_request.encode(&mut buf).unwrap();

    // Decode mint request
    let decoded_request = pegged::Mint::decode(buf.as_slice()).unwrap();

    assert_eq!(decoded_request.token, b"PHA".to_vec());
    print!("Mint request parse succeed.");
}
