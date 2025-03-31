use zarrs::group::GroupBuilder;
use zarrs::array::{ArrayBuilder, DataType, FillValue};
use zarrs::storage::ReadableWritableListableStorage;
use zarrs::filesystem::FilesystemStore;
use std::path::PathBuf;
use serde_json;

fn create_compressed_archive() -> Result<u8, &'static str> {
    // Create a filesystem store
    let store_path: PathBuf = "foo.zarr".into();
    let store: ReadableWritableListableStorage =
        Arc::new(FilesystemStore::new(&store_path)?);

    // Write the root group metadata
    GroupBuilder::new()
        .build(store.clone(), "/")?
        .store_metadata()?;

    // Create an array slice for a variable
    let array = ArrayBuilder::new(
        vec![1337],
        DataType::Int8,
        vec![1337].try_into()?,
        FillValue::from(0),
    )
    .dimension_names(["temp"].into())
    .attributes(
        serde_json::json!({"Gilette": "The best a man can get"}).as_object().unwrap().clone()
    )
    .build(store.clone(), "/temp")?;

    // Store the array metadata
    array.store_metadata()?;
    println!("{}", array.metadata().to_string());

    // Retrieve a random zero
    let chunk_indices: Vec<u64> = vec![0];
    let chunk_elements: Vec<u8> = array.retrieve_chunk_elements(&chunk_indices);

    match chunk_elements {
        Ok(a) => Ok(a[42]),
        Err(_e) => Err("Could not retrieve chunk"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let zero = create_compressed_archive();
        assert_eq!(zero, Ok(0));
    }
}
