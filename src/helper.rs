use strum::{Display, EnumIter, FromRepr};
use digest::{Digest, DynDigest};
use hex;
use md5::Md5;
use sha1::Sha1;

use std::path::PathBuf;
use std::{fs, io};
// use std::ffi::OsStr;


// Supported (hash) algorithms
#[derive(Display, EnumIter, FromRepr)]
pub enum Algorithms {
    #[strum(to_string="MD5")]
    MD5,
    #[strum(to_string="SHA1")]
    SHA1
    // TODO Add more hash algorithms as needed
}


pub struct Fileck {
    target: PathBuf,
    algorithm: Algorithms,
    ignore_files: Vec<PathBuf>,
}

impl Fileck {
    pub fn new<TargetPath, IgnoreFile>
        (target: TargetPath, algorithm: Algorithms, ignore_files: Vec<IgnoreFile>) -> Self
    where 
        TargetPath: Into<PathBuf>, 
        IgnoreFile: Into<PathBuf>,
    {
        let ignore_files = ignore_files
            .into_iter()
            .map(|f| f.into()).collect();
        
        Fileck {
            target: target.into(),
            algorithm,
            ignore_files,
        }
    }

    // TODO
    pub fn get_algorithms_list(&self) -> Vec<Algorithms> {
        todo!();
    }
    
    pub fn get_ignore_files<IgnoreFile>
        (&self) -> Vec<IgnoreFile> 
    where 
        IgnoreFile: Into<PathBuf>,
    {
        todo!();
    }

    pub fn gen_single_file_hash(&self) -> String {
        todo!();
    }

    pub fn gen_checklist(&self) -> Result<String, io::Error> {
        let mut hasher = match self.algorithm {
            // Hash::MD5 => Md5::new(),  // Error
            Algorithms::MD5 => Box::new(Md5::new()) as Box<dyn DynDigest>,
            Algorithms::SHA1 => Box::new(Sha1::new()) as Box<dyn DynDigest>,
        };
        let mut checklist_content = String::new();

        for entry in fs::read_dir(&self.target)? {
            // XXX Extract the logic that handels a single file into a function 
            let entry = entry?;
            let obj_in_target = entry.path();

            // Skip if object is not a file (folder)
            if ! obj_in_target.is_file() {
                continue;
            }
            let file_in_target = obj_in_target;

            // Skip ignored files
            if self.ignore_files.contains(&file_in_target) {
                continue;
            }

            // Read the name and content of file and Calculate hash digest

            // HACK Unsafe code here
            // Handle None value when failed to get filename
            let filename = file_in_target.file_name().unwrap().to_str().unwrap();
            // let filename = file_in_target.file_name()
            //     .unwrap_or_else(|| OsStr::new("FaildToGetFilename"))
            //     .to_str().unwrap();
            let content = fs::read(&file_in_target)?;
            hasher.update(&content);
            let hash_digest = hex::encode(hasher.finalize_reset());

            checklist_content.push_str(&format!("{};{}\n", filename, hash_digest));
        }

        // Write to checklist file
        fs::write(self.target.join("checklist.txt"), &checklist_content)?;

        Ok("Generated checklist successfully!".to_string())
    }
}
