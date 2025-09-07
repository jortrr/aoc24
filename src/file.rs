use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

/// Read file located in resources, split the file by line,
/// and return the resulting vector of lines.
pub fn read_file_by_line(
    file_name: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let path: PathBuf =
        fs::canonicalize(Path::new("resources/"))?
            .join(file_name);
    let file: Vec<String> = fs::read_to_string(path)?
        .split('\n')
        .map(str::to_string)
        .collect();
    Ok(file)
}
