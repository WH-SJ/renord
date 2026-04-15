use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use glob::glob;
use crate::SortMod;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub size: u64,
}

impl FileEntry {
    fn new(path: PathBuf, name: String, modified: SystemTime, created: SystemTime, size: u64) -> Self {
        Self { path, name, modified, created, size }
    }

    pub fn rename(&mut self, new_name: &str) -> std::io::Result<()> {
        let new_path = self
            .path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(new_name);
        fs::rename(&self.path, &new_path)?;
        self.path = new_path;
        Ok(())
    }
}

pub fn load_files(path: &str, ext: &Option<String>, sort_mod: &Option<SortMod>) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
    let pattern = match ext {
        Some(e) => format!("{path}/*.{e}"),
        None    => format!("{path}/*"),
    };

    let entries = glob(&pattern)?.filter_map(|res| res.ok());

    let mut res: Vec<FileEntry> = entries
        .filter_map(|path| {
            let meta = path.metadata().ok()?;
            
            let name = path.file_name()?.to_str()?.to_string();
            let modified = meta.modified().ok()?;
            let created = meta.created().ok()?;
            let size = meta.len();

            Some(FileEntry::new(path, name, modified, created, size))
        })
        .collect();

    if let Some(s) = sort_mod {
        match s {
            SortMod::Name     => res.sort_by_key(|f| f.path.file_name().map(|n| n.to_owned())),
            SortMod::Size     => res.sort_by_key(|f| f.size),
            SortMod::Modified => res.sort_by_key(|f| f.modified),
            SortMod::Created  => res.sort_by_key(|f| f.created),
        }
    }

    Ok(res)
}