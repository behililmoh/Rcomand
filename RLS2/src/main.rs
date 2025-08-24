use clap::Parser;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};

#[derive(Parser, Debug)]
#[command(version, about = "A simple ls clone in Rust", long_about = None)]
struct Args {
    /// List all files, including hidden ones.
    #[arg(short = 'a', long)]
    all: bool,

    /// Use a long listing format.
    #[arg(short = 'l', long)]
    long: bool,

    /// List subdirectories recursively.
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Target files or directories.
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    for path in &args.paths {
        if args.recursive {
            list_recursive(path, &args, 0)?;
        } else if args.long {
            list_long_format(path, &args)?;
        } else {
            list_short_format(path, &args)?;
        }
    }

    Ok(())
}

fn list_short_format(path: &Path, args: &Args) -> io::Result<()> {
    // Gérer l'affichage pour le chemin donné
    println!("{}:", path.to_string_lossy());
    
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        if !args.all && file_name.to_string_lossy().starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let is_dir = metadata.is_dir();

        if is_dir {
            print!("\x1B[1;34m{}\x1B[0m/  ", file_name.to_string_lossy());
        } else {
            print!("{}  ", file_name.to_string_lossy());
        }
    }
    println!("\n");
    Ok(())
}

fn list_long_format(path: &Path, args: &Args) -> io::Result<()> {
    println!("{}:", path.to_string_lossy());

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        if !args.all && file_name.to_string_lossy().starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let size = metadata.len();
        let is_dir = metadata.is_dir();
        
        let file_type_char = if is_dir { 'd' } else { '-' };
        
        // Correction de l'erreur E0308 : Convertir SystemTime en DateTime<Local>
        let modified_time = metadata.modified()?;
        let datetime: DateTime<Local> = modified_time.into();
        let formatted_time = datetime.format("%b %e %H:%M").to_string();

        let name_to_display = if is_dir {
            format!("\x1B[1;34m{}\x1B[0m/", file_name.to_string_lossy())
        } else {
            file_name.to_string_lossy().to_string()
        };
        
        // Simplification pour l'exemple
        println!("{}-rwx------ 1 user group {} {} {}", file_type_char, size, formatted_time, name_to_display);
    }
    println!("\n");
    Ok(())
}

fn list_recursive(path: &Path, args: &Args, depth: usize) -> io::Result<()> {
    let prefix = "  ".repeat(depth);
    
    println!("{}{}:", prefix, path.to_string_lossy());

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();

            if !args.all && file_name.to_string_lossy().starts_with('.') {
                continue;
            }

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
    }

    Ok(())
}