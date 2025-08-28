# rust-ls

Un outil en ligne de commande (CLI) puissant, conçu pour lister le contenu des répertoires, avec le support de plusieurs options avancées. Ce projet est une implémentation de la commande `ls` écrite en **Rust**.

---

## Fonctionnalités

-   **Affichage Standard :** Liste les fichiers et dossiers du répertoire courant.
-   **Options de Base (`-a`, `-l`) :**
    -   **`-l` (format long) :** Affiche des détails comme la taille, la date de modification et le type (fichier ou dossier). Les autorisations sont incluses à titre d'exemple (`-rwx------`).
    -   **`-a` (tous les fichiers) :** Inclut les fichiers et dossiers cachés (ceux dont le nom commence par un point).
-   **Affichage Récursif (`-R`) :** Explore les sous-dossiers de manière récursive, affichant une arborescence complète.
-   **Format de Sortie JSON (`--json`) :** Permet de générer une sortie structurée en JSON, idéale pour les scripts ou les applications qui traitent des données.
-   **Tri des Fichiers (`--sort`) :** Triez les résultats selon différents critères :
    -   `--sort name` (par défaut) : Tri par nom de fichier.
    -   `--sort size` : Tri par taille de fichier.
    -   `--sort time` : Tri par date de dernière modification.
-   **Support des Chemins Multiples :** Vous pouvez spécifier un ou plusieurs chemins de répertoires à lister.

---

## Prérequis

Pour compiler et exécuter ce projet, vous devez avoir installé [Rust](https://www.rust-lang.org/fr) sur votre système. Ce projet utilise plusieurs "crates" (bibliothèques Rust) qui doivent être ajoutées à votre fichier `Cargo.toml`.

```bash
cargo build --release
cargo run -- -l 
cargo run -- -a -R
cargo run -- --sort size --json
cargo run -- mon_dossier /home/utilisateur/documents

Voici les dépendances nécessaires :

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"# rust-ls 
