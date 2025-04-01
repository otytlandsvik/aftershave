use serde_json;
use std::path::PathBuf;
use std::sync::Arc;
use zarrs::array::{ArrayBuilder, DataType, FillValue};
use zarrs::filesystem::FilesystemStore;
use zarrs::group::GroupBuilder;
use zarrs::storage::ReadableWritableListableStorage;

pub fn create_compressed_archive() -> Result<i8, Box<dyn std::error::Error>> {
    // Create a filesystem store
    let store_path: PathBuf = "foo.zarr".into();
    let store: ReadableWritableListableStorage = Arc::new(FilesystemStore::new(&store_path)?);

    // Write the root group metadata
    GroupBuilder::new()
        .build(store.clone(), "/")?
        .store_metadata()?;

    // Create an array slice for a variable
    let array = ArrayBuilder::new(
        vec![1337],
        DataType::Int8,
        vec![1337].try_into()?,
        FillValue::from(0i8),
    )
    .dimension_names(["temp"].into())
    .attributes(
        serde_json::json!({"Gilette": "The best a man can get"})
            .as_object()
            .unwrap()
            .clone(),
    )
    .build(store.clone(), "/temp")?;

    // Store the array metadata
    array.store_metadata()?;
    println!("{}", array.metadata().to_string());

    // Retrieve a random zero
    let chunk_indices: Vec<u64> = vec![0];
    let chunk_elements: Vec<i8> = array.retrieve_chunk_elements(&chunk_indices)?;

    Ok(chunk_elements[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let zero = create_compressed_archive();
        match zero {
            Ok(z) => assert_eq!(z, 0),
            Err(_e) => panic!(),
        }
    }
}
