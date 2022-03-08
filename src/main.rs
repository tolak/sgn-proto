mod binding;

use protobuf::Message;
use binding::protos::pegged;


fn main() {
    // Mint
    let mut mint_request = pegged::Mint::new();
    mint_request.token = b"PHA".to_vec();
    mint_request.account = b"Alice".to_vec();
    mint_request.amount = b"10000".to_vec();
    mint_request.depositor = b"Bob".to_vec();
    mint_request.ref_chain_id = 1;
    mint_request.ref_id = b"ref id".to_vec();

    let out_bytes: Vec<u8> = mint_request.write_to_bytes().unwrap();

    // Decode mint request
    let decoded_request = pegged::Mint::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(decoded_request.token, b"PHA".to_vec());
    print!("Mint request parse succeed.");
}
