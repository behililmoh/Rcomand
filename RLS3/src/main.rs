use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use serde::Serialize;

/// Outil CLI simulant la commande `ls` avec options avancées.
#[derive(Parser, Debug)]
#[command(version, about = "Un clone simplifié de ls en Rust")]
struct Args {
    /// Affiche tous les fichiers, y compris les fichiers cachés.
    #[arg(short = 'a', long)]
    all: bool,

    /// Utilise le format long.
    #[arg(short = 'l', long)]
    long: bool,

    /// Liste les sous-dossiers récursivement.
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Affiche en JSON.
    #[arg(long)]
    json: bool,

    /// Trie les fichiers (par nom, taille ou date).
    #[arg(long, value_enum, default_value = "name")]
    sort: SortBy,

    /// Chemins à explorer.
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SortBy {
    Name,
    Size,
    Time,
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    size: u64,
    modified: String,
    is_dir: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    for path in &args.paths {
        if args.recursive {
            list_recursive(path, &args, 0)?;
        } else {
            list_directory(path, &args)?;
        }
    }

    Ok(())
}

fn list_directory(path: &Path, args: &Args) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    match args.sort {
        SortBy::Name => entries.sort_by_key(|e| e.file_name()),
        SortBy::Size => entries.sort_by_key(|e| e.metadata().map(|m| m.len()).unwrap_or(0)),
        SortBy::Time => entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH)),
    }

    if args.json {
        let infos: Vec<FileInfo> = entries.iter().filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let modified: DateTime<Local> = metadata.modified().ok()?.into();
            Some(FileInfo {
                name: entry.file_name().to_string_lossy().to_string(),
                size: metadata.len(),
                modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
                is_dir: metadata.is_dir(),
            })
        }).collect();

        println!("{}", serde_json::to_string_pretty(&infos).unwrap());
    } else {
        println!("{}:", path.to_string_lossy());
        for entry in entries {
            let metadata = entry.metadata()?;
            let file_name = entry.file_name();
            let is_dir = metadata.is_dir();
            let size = format_size(metadata.len());
            let modified: DateTime<Local> = metadata.modified()?.into();
            let time = modified.format("%b %e %H:%M").to_string();

            if args.long {
                let type_char = if is_dir { 'd' } else { '-' };
                let name = if is_dir {
                    format!("\x1B[1;34m{}\x1B[0m/", file_name.to_string_lossy())
                } else {
                    file_name.to_string_lossy().to_string()
                };
                println!("{}-rwx------ 1 user group {:>8} {} {}", type_char, size, time, name);
            } else {
                if is_dir {
                    print!("\x1B[1;34m{}\x1B[0m/  ", file_name.to_string_lossy());
                } else {
                    print!("{}  ", file_name.to_string_lossy());
                }
            }
        }
        if !args.long {
            println!();
        }
    }

    Ok(())
}

fn list_recursive(path: &Path, args: &Args, depth: usize) -> io::Result<()> {
    let prefix = "  ".repeat(depth);
    println!("{}{}:", prefix, path.to_string_lossy());

    let entries = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect::<Vec<_>>();

    for entry in entries {
        let file_name = entry.file_name();
        let metadata = entry.metadata()?;
        let is_dir = metadata.is_dir();
        let path_to_list = entry.path();

        if is_dir {
            println!("{}├── \x1B[1;34m{}\x1B[0m/", prefix, file_name.to_string_lossy());
            list_recursive(&path_to_list, args, depth + 1)?;
        } else {
            println!("{}├── {}", prefix, file_name.to_string_lossy());
        }
    }

    Ok(())
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

