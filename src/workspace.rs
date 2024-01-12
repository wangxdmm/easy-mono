use std::{collections::HashMap, path::PathBuf};

use crate::package::Package;

pub struct Workspace {
    pub globs: Vec<String>,
    pub packages: HashMap<String, Package>,
}

impl Workspace {
    pub fn new(dir: PathBuf) -> Self {
        Workspace {
            globs: vec![],
            packages: HashMap::new(),
        }
    }
}
