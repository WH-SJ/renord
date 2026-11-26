use crate::SortMod;
use glob::glob;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FsEntry {
    pub path: PathBuf,
    pub name: String,
    pub entry_type: EntryType,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub size: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum EntryType {
    File,
    Dir,
}

impl FsEntry {
    fn new(
        path: PathBuf,
        name: String,
        entry_type: EntryType,
        modified: SystemTime,
        created: SystemTime,
        size: u64,
    ) -> Self {
        Self {
            path,
            name,
            entry_type,
            modified,
            created,
            size,
        }
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

pub fn load_fs(
    path: &str,
    entry_type: EntryType,
    ext: &Option<String>,
    sort_mod: &Option<SortMod>,
) -> Result<Vec<FsEntry>, Box<dyn std::error::Error>> {
    let mut pattern = path.to_string();
    if pattern.ends_with("/") {
        pattern.push('*');
    } else if !pattern.ends_with("/*") {
        pattern.push_str("/*");
    }

    let entries = match entry_type {
        EntryType::File => {
            if let Some(e) = ext {
                pattern = format!("{pattern}/*.{e}");
            }

            glob(&pattern)?.filter_map(|p| p.ok()).collect::<Vec<_>>()
        }
        EntryType::Dir => glob(&pattern)?.filter_map(|p| p.ok()).collect::<Vec<_>>(),
    };

    let mut res: Vec<FsEntry> = entries
        .into_iter()
        .filter_map(|path| {
            let meta = path.metadata().ok()?;
            let name = path.file_name()?.to_str()?.to_string();
            let modified = meta.modified().ok()?;
            let created = meta.created().ok()?;
            let size = meta.len();

            Some(FsEntry::new(
                path, name, entry_type, modified, created, size,
            ))
        })
        .collect();

    if let Some(s) = sort_mod {
        match s {
            SortMod::Name => res.sort_by_key(|f| f.path.file_name().map(|n| n.to_owned())),
            SortMod::Size => res.sort_by_key(|f| f.size),
            SortMod::Modified => res.sort_by_key(|f| f.modified),
            SortMod::Created => res.sort_by_key(|f| f.created),
        }
    }

    Ok(res)
}
