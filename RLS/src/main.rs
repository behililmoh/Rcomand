use clap::Parser;
//use std::fs::{self, DirEntry, Metadata};
use std::fs::{self};
use std::io;
//use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about = "A simple ls clone in Rust", long_about = None)]
struct Args {
    /// List all files, including hidden ones.
    #[arg(short = 'a', long)]
    all: bool,

    /// Use a long listing format.
    #[arg(short = 'l', long)]
    long: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
    // Gérer l'option `-l` ou `-a`
    if args.long {
        list_long_format(args.all)?;
    } else {
        list_short_format(args.all)?;
    }

    Ok(())
}

fn list_short_format(show_all: bool) -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        
        // Gérer les fichiers cachés (commençant par '.')
        if !show_all && file_name.to_string_lossy().starts_with('.') {
            continue;
        }

        // Différencier les dossiers des fichiers
        let path = entry.path();
        if path.is_dir() {
            println!("\x1B[1;34m{}\x1B[0m/", file_name.to_string_lossy()); // Couleur bleue pour les dossiers
        } else {
            println!("{}", file_name.to_string_lossy());
        }
    }
    Ok(())
}

fn list_long_format(show_all: bool) -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();

        if !show_all && file_name.to_string_lossy().starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;

        // Afficher la taille, le nom, et le type de fichier
        let size = metadata.len();
        let is_dir = metadata.is_dir();
        
        if is_dir {
            println!("d\t{}\t\x1B[1;34m{}\x1B[0m/", size, file_name.to_string_lossy());
        } else {
            println!("-\t{}\t{}", size, file_name.to_string_lossy());
        }
    }
    Ok(())
}