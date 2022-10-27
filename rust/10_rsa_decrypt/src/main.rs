// references: https://www.simplilearn.com/tutorials/cryptography-tutorial/aes-encryption#:~:text=The%20AES%20Encryption%20algorithm%20(also,together%20to%20form%20the%20ciphertext.
use clap::Parser;
use rsa::{pkcs1::DecodeRsaPrivateKey, PaddingScheme, RsaPrivateKey};
use std::{fs, str};

#[derive(Parser)]
#[command(name = "RSA decryption program")]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(short, long, help = "The private key used for decryption")]
    privkey: String,
    #[arg(short, long, help = "The message file to be decrypted")]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let pem_file = fs::read_to_string(&cli.privkey).unwrap();
    let priv_key = RsaPrivateKey::from_pkcs1_pem(&pem_file).unwrap();
    // let pub_key = RsaPublicKey::from(&priv_key);

    let pass_file = fs::read(&cli.file).unwrap();
    let padding = PaddingScheme::PKCS1v15Encrypt;
    let dec_data = priv_key.decrypt(padding, &pass_file).unwrap();

    println!("{}", str::from_utf8(&dec_data).unwrap());
}
