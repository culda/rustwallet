use ethers::signers::{coins_bip39::English, MnemonicBuilder, Signer};
fn main() {
    let phrase = "";

    for i in 0..10 {
        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/60'/0'/0/{}", i).as_str())
            .unwrap();

        println!("{:?}", builder.build().unwrap().address());
    }
}
