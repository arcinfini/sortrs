/*

    defines the trait that is used to implement sorting methods.
    this also holds the methods for sorting.

*/

use std::path::PathBuf;
use crate::error::SortResult;

pub trait SortMethod {
    fn build_name(&self, path: &PathBuf) -> SortResult<PathBuf>;
}

pub struct FileExtensionSorter;


/*

    pulls out the file extension to then shove it in a directory of the file
    extension
*/
impl SortMethod for FileExtensionSorter {
    fn build_name(&self, path: &PathBuf) -> SortResult<PathBuf> {
        
        let extension = path.extension().ok_or_else(
            || format!("{} has an invalid extension", path.to_string_lossy())
        )?;
        let name = path.file_name().ok_or_else(
            || format!("{} has an invalid name", path.to_string_lossy())
        )?;

        let mut parent = path.parent().ok_or_else(
            || format!("{} has an invalid parent", path.to_string_lossy())
        )?.to_path_buf();

        let mut name_buf = name.to_os_string();
        name_buf.push(".");
        name_buf.push(&extension);

        parent.push(extension);
        parent.push(name_buf);

        Ok(parent)
    }
}