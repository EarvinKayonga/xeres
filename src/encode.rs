#[macro_use]
extern crate failure;
extern crate rayon;

#[macro_use]
extern crate lazy_static;

mod cipher;
mod interactif;

use failure::Error;

fn main() -> Result<(), Error> {
    let text = interactif::get_text().map_err(|err| format_err!("could get text: {}", err))?;
    let key = interactif::get_key().map_err(|err| format_err!("could get key {}", err))?;

    println!("cipher text: {}", cipher::encode(text, key));

    Ok(())
}
