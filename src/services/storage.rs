use chrono::Utc;
use env::Env;
use rand::{self, distributions::Alphanumeric, Rng};
use std::fs::File;
use std::fs::{create_dir_all, remove_file};
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct StorageService;

impl StorageService {
    pub fn store(directory: String, bytes: &[u8]) -> Result<String, std::io::Error> {
        #[cfg(test)]
        let directory = String::from("test");

        let timestamp = Utc::now().to_string();
        let random_bytes: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect();

        let file_name = format!(
            "{timestamp}_{random_bytes}",
            timestamp = timestamp,
            random_bytes = random_bytes
        );

        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        let mut file = match File::create(Path::new(&path)) {
            Ok(file) => Ok(file),
            Err(_) => {
                // Its possible the directory doesn't exist yet
                create_dir_all(format!(
                    "{}/{directory}",
                    Env::storage_dir(),
                    directory = directory,
                ))?;

                File::create(Path::new(&path))
            }
        }?;
        file.write(bytes)?;

        file.flush()?;

        Ok(file_name)
    }

    pub fn read(directory: String, file_name: String) -> Result<Vec<u8>, std::io::Error> {
        #[cfg(test)]
        let directory = String::from("test");

        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        let mut file = File::open(Path::new(&path))?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    pub fn delete(directory: String, file_name: String) -> Result<(), std::io::Error> {
        #[cfg(test)]
        let directory = String::from("test");

        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        remove_file(Path::new(&path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        dotenv::dotenv().expect("Missing .env file");

        let bytes: &[u8] = &[];
        let directory = String::from("test");

        let result = StorageService::store(directory.clone(), bytes).unwrap();

        let path = format!("{}/{}/{}", Env::storage_dir(), directory, result);

        let mut file = File::open(Path::new(&path)).unwrap();

        let mut buffer = Vec::new();
        file.read(&mut buffer).unwrap();

        assert_eq!(bytes, buffer.as_slice());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read() {
        dotenv::dotenv().expect("Missing .env file");

        let bytes: &[u8] = &[];
        let directory = String::from("test");
        let file_name = String::from("read");

        let path = format!(
            "{}/{directory}/{file_name}",
            Env::storage_dir(),
            directory = directory,
            file_name = &file_name
        );

        let mut file = File::create(Path::new(&path)).unwrap();
        file.write(bytes).unwrap();

        file.flush().unwrap();

        let result = StorageService::read(directory, file_name).unwrap();

        assert_eq!(bytes, result.as_slice());

        std::fs::remove_file(path).unwrap();
    }
}
