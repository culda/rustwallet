use std::convert::TryInto;

use ethers::{
    core::types::H256,
    prelude::k256::ecdsa::SigningKey,
    signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet},
};
fn main() {
    let phrase = ""; // <-- phrase here

    for i in 0..10 {
        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/60'/0'/0/{}", i).as_str())
            .unwrap();

        let wallet = builder.build().unwrap();

        let address = wallet.address();
        let signer = wallet.signer();
        let signer_bytes: [u8; 32] = signer
            .to_bytes()
            .as_slice()
            .try_into()
            .expect("wrong length");

        println!("{:#x}\n{:#x}\n", address, H256::from(signer_bytes));
    }
}

#[allow(dead_code)]
fn get_random_wallet() -> Wallet<SigningKey> {
    let mut rng = rand::thread_rng();
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(24)
        .build_random(&mut rng)
        .unwrap();
    wallet
}
