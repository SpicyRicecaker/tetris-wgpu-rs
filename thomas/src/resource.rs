use std::{fs::File, path::PathBuf};

pub struct ResourceManager {
    // Contains path to resource folder
    path: PathBuf,
}

impl ResourceManager {
    // Pathbuf empty initially
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Sets the root folder for resources
    /// Currently not dynamic 'cause I don't see a reason for it to be
    pub fn set_res_folder(&mut self, path: PathBuf) {
        self.path = path;
    }

    pub fn load_file(&mut self, path: &str) -> Result<File, std::io::Error> {
        // Try getting a path to the resource
        self.path.push(path);
        let file = File::open(&self.path)?;
        self.path.pop();
        Ok(file)
    }

    /// Loads raw data from file
    pub fn load_data(&mut self, resource: &str) -> Result<Vec<u8>, std::io::Error> {
        // Try getting a path to the resource
        self.path.push(resource);
        let res = std::fs::read(&self.path);
        self.path.pop();
        res
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }
}
