use crate::fs_manager::{EntryType, load_fs};
use clap::{Parser, Subcommand, ValueEnum};
use regex::Regex;
use std::fs;

mod fs_manager;

#[derive(Subcommand)]
enum Command {
    /// 按名字列表重新排序重命名
    /// Reorder and rename by name list
    Reorder {
        #[arg(short, long)]
        names_file: String,

        #[arg(short, long)]
        sort_mod: Option<SortMod>,
    },

    /// 批量替换文件名中的字符串
    /// Replace strings in file names in bulk
    Replace {
        #[arg(short, long)]
        from: String,

        #[arg(short, long)]
        to: String,

        #[arg(short, long)]
        regex: bool,
    },
}

#[derive(Parser)]
#[command(
    name = "renord",
    subcommand_help_heading = "Commands",
    flatten_help = true
)]
struct Cli {
    dir_path: String,

    /// 只预览，不实际执行重命名
    /// Only preview, no actual renaming
    #[arg(long, global = true)]
    dry_run: bool,

    /// 忽略大小写（仅对 replace 子命令有效）
    /// Ignore case (only valid for replace subcommand)
    #[arg(long, global = true)]
    ignore_case: bool,

    /// 只处理特定扩展名，不传则匹配所有文件
    /// Only specific extensions are processed, and if not, all files are matched
    #[arg(short, long, global = true)]
    ext: Option<String>,

    /// 匹配文件夹
    /// match dir, default match file
    #[arg(short, long, global = true)]
    dir: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(ValueEnum, Clone)]
enum SortMod {
    Name,
    Size,
    Modified,
    Created,
}

#[derive(ValueEnum, Clone)]
enum Type {
    File,
    Dir,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let entry_type = if cli.dir {
        EntryType::Dir
    } else {
        EntryType::File
    };

    match &cli.command {
        Command::Reorder {
            names_file,
            sort_mod,
        } => {
            let mut entrys = load_fs(&cli.dir_path, entry_type, &cli.ext, sort_mod)?;
            let names: Vec<String> = fs::read_to_string(names_file)?
                .lines()
                .map(String::from)
                .collect();

            if entrys.len() != names.len() {
                eprintln!(
                    "警告：找到 {} 个文件，但只有 {} 个名字，仅重命名前 {} 个",
                    entrys.len(),
                    names.len(),
                    entrys.len().min(names.len())
                );
            }

            for (i, (entry, name)) in entrys.iter_mut().zip(names).enumerate() {
                let new_name = match entry.entry_type {
                    EntryType::File => {
                        let ext = entry
                            .path
                            .extension()
                            .and_then(|e| e.to_str())
                            .expect("file ext name");
                        &format!("{}.{}.{}", i + 1, name, ext)
                    }
                    EntryType::Dir => &format!("{}.{}", i + 1, name),
                };

                println!("{:#?} rename to --> {}", entry.path, new_name);

                if !cli.dry_run {
                    entry.rename(new_name)?
                }
            }
        }
        Command::Replace { from, to, regex } => {
            let mut entrys = load_fs(&cli.dir_path, entry_type, &cli.ext, &None)?;

            let re = if cli.ignore_case || *regex {
                let pattern = if cli.ignore_case {
                    format!("(?i){from}")
                } else {
                    from.clone()
                };

                Some(Regex::new(&pattern)?)
            } else {
                None
            };

            for entry in &mut entrys {
                let file_name = &entry.name;

                // let new_name = match &re {
                //     Some(r) => r.replace_all(file_name, to).to_string(),
                //     None => file_name.replace(from, to),
                // };

                let new_name = match &re {
                    Some(r) if r.is_match(file_name) => r.replace_all(file_name, to).to_string(),
                    Some(_) => {
                        println!("No match: {}", file_name);
                        continue;
                    }
                    None => file_name.replace(from, to),
                };

                println!("{:#?} rename to --> {}", file_name, new_name);

                if !cli.dry_run {
                    entry.rename(&new_name)?
                }
            }
        }
    }

    Ok(())
}
