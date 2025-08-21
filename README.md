# rust-ls

Un clone simple et rapide de la commande `ls` pour les systèmes de fichiers. Ce projet est une implémentation de la commande `ls` en utilisant le langage de programmation Rust, avec le support de plusieurs options de base.

## Fonctionnalités

-   **Affichage standard :** Liste les fichiers et dossiers du répertoire courant.
-   **Affichage détaillé (`-l`) :** Affiche la taille des fichiers et le type (fichier ou dossier) en plus du nom.
-   **Inclusion des fichiers cachés (`-a`) :** Affiche tous les fichiers, y compris ceux qui commencent par un point (`.`).
-   **Combinaison des options (`-la`) :** Permet d'utiliser les deux options ensemble pour un affichage détaillé incluant les fichiers cachés.
-   **Coloration des dossiers :** Les dossiers sont affichés en bleu pour une meilleure lisibilité.

## Prérequis

Pour compiler et exécuter ce projet, vous devez avoir installé Rust sur votre système. Si ce n'est pas le cas, vous pouvez l'installer en utilisant `rustup` :



# rust-ls
A simple and fast clone of the ls command for file systems. This project is an implementation of the ls command using the Rust programming language, with support for several basic options.

## Features
- **Standard Display:** Lists files and directories in the current directory.

- **Detailed Display (`-l`):** Shows file size and type (file or directory) in addition to the name.

- **Include Hidden Files (`-a`):** Displays all files, including those that start with a dot (.).

- **Option Combination (`-la`a):**  both options to be used together for a detailed display that includes hidden files.

Directory Coloring: Directories are shown in blue for better readability.

## Prerequisites
To compile and run this project, you must have Rust installed on your system. If not, you can install it using rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh