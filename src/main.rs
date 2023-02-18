use std::io::Error;

mod config;

fn main() -> Result<(), Error> {
    let config = config::load_config()?;
    println!("{:#?}", config);
    Ok(())
}
