use futures_util::TryStreamExt;

// use crate::shared::common::StorageService;

use std::{fs::{self, File}, io::{Error, Write}};

// use async_trait::async_trait;

#[derive(Clone)]
pub struct FileStore {
    base_path: String
}

#[allow(dead_code)]
impl FileStore {
    pub fn new(base_path: String) -> FileStore {
        FileStore {base_path}
    }
// }

// #[async_trait(?Send)]
// impl StorageService for FileStore {
    pub async fn save_file(&self, path: &String, name: &String, input: &mut futures_util::stream::IntoStream<actix_multipart::Field>) -> Result<(), Error> {
        let mut file = File::create(format!("{}/{}/{}", self.base_path,path,name)).expect("Failed to create file");

        // Field in turn is a stream of Bytes object
        while let Ok(chunk) = input.try_next().await {
            if let Some(chunk) = chunk {
                file.write_all(&chunk)?;
            } else {
                log::info!("Finished writing file {}", name);
                break;
            }
        }
        
        Ok(())
    }
    pub fn retrieve_file(&self, _path: String, _name: String) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    pub fn create_folder(&self, path: String) -> Result<(), Error> {
        fs::create_dir_all(format!("{}/{}",self.base_path, path))
    }

    fn list_file_names(&self, _path: String) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

    fn list_folder_names(&self, _path: String) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

}