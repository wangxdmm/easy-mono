use std::path::PathBuf;

pub struct Package {
    pub name: String,
    pub loc: PathBuf,
    pub update_files: Vec<String>,
    pub ignore: Vec<String>,
    pub deps: Vec<String>,
}
