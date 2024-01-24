/*

    sortrs:

    sort files within a provided directory

*/
mod error;
mod methods;

use std::fs::{read_dir, rename};
use std::path::PathBuf;
use crate::error::SortResult;

struct Sorter {
    dir: PathBuf,
    method: Box<dyn methods::SortMethod>,
    // failed: Vec<PathBuf>,
    // succeeded: Vec<PathBuf>
}

impl Sorter {
    pub fn new(dir: PathBuf, method: Box<dyn methods::SortMethod>) -> Self {
        Self { 
            dir, 
            method
            // failed: Vec::new(), succeeded: Vec::new() 
        }
    }
    
    fn rename_file(&self, cur_loc: &PathBuf) -> SortResult<PathBuf> {

        let new_loc = self.method.build_name(&cur_loc)?;
        rename(cur_loc, &new_loc)?;

        // allow verbose loggings to display result as files are being sorted

        Ok(new_loc)
    }

    /*
    
        Maps through the files in the provided directory on struct creation,
        skips over child directories and then renames the files.
    */
    pub fn filter_dir(&self) -> SortResult<Vec<PathBuf>> {
        let dir = &self.dir;

        let successors = read_dir(dir)?.into_iter()
            .filter_map(|r_entry| r_entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .map(|path| self.rename_file(&path))
            .filter_map(|r_path| r_path.ok())
            .collect();

        Ok(successors)
    }
}

fn main() -> SortResult<()>{
    println!("Hello, world!");

    let sorter = Sorter::new(
        PathBuf::from("/invalid"),
        Box::new(methods::FileExtensionSorter)
    );

    let successors = sorter.filter_dir()?;
    println!("Sucessors: {:?}", successors);

    Ok(())
}
