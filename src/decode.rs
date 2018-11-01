#[macro_use]
extern crate failure;
extern crate rayon;

#[macro_use]
extern crate lazy_static;

mod cipher;
mod interactif;

use failure::Error;

fn main() -> Result<(), Error> {
    let cipher_text = interactif::get_cipher_text()
        .map_err(|err| format_err!("could get cipher text: {}", err))?;
    let key = interactif::get_key().map_err(|err| format_err!("could get key: {}", err))?;

    println!("decoded text: {}", cipher::decode(cipher_text, key));

    Ok(())
}
