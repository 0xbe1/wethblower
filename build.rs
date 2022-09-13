use ethers::contract::Abigen;
use eyre::Result;

fn main() -> Result<()> {
    Abigen::new("WETH", "./abi/WETH.json")?
        .generate()?.write_to_file("src/abi/weth.rs")?;
    Ok(())
}