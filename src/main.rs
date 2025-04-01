use aftershave_lib::create_compressed_archive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zero = create_compressed_archive()?;
    println!("{}", zero);
    Ok(())
}
