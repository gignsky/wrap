use clap::Parser;
use std::fs::File;
use std::path::Path;
use tar::Builder;

#[derive(Parser, Debug)]
#[clap(author = "Maxwell Rupp", version, about)]
/// Application configuration
struct Args {
    /// Print Verbose output
    #[arg(short = 'v')]
    verbose: bool,

    /// Remove folders after tarballing
    #[arg(short = 'r', long = "remove")]
    remove: bool,

    /// Dry run - List folders to be tarballed but do not create tarballs
    #[arg(short = 'd', long = "dry-run")]
    dry_run: bool,

    /// Test Argument
    #[arg(short = 't', long = "test")]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let current_dir = find_current_dir();

    let tarball_names_and_paths = pathfinder(args.verbose, current_dir);

    tarballer(
        args.dry_run,
        args.verbose,
        args.remove,
        tarball_names_and_paths,
        current_dir,
    );
}
fn find_current_dir() -> &'static Path {
    let current_dir = Path::new(".");
    current_dir
}

/// Finds all folders in the current directory and returns a hashmap of tarball names and paths
fn pathfinder(
    verbose: bool,
    current_dir: &Path,
) -> std::collections::HashMap<String, std::path::PathBuf> {
    // find current directory
    if verbose {
        println!("Current directory: {:?}", current_dir);
    }

    // start vec of folder paths
    let mut folder_paths = Vec::new();

    // filter paths to only include folders
    let paths = std::fs::read_dir(current_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if verbose {
            println!("Path: {:?}", path);
        }
        if path.is_dir() {
            if verbose {
                println!("Folder path detected: {:?}", path);
            }
            folder_paths.push(path);
        }
    }

    // start new hashmap for tarball names
    let mut tarball_names_and_paths = std::collections::HashMap::new();

    // iterate over folder paths and add to hashmap with {folderName}.tar as key and path as value
    for folder_path in folder_paths {
        let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
        if verbose {
            println!("Folder name: {:?}", folder_name);
        }
        let tarball_name = format!("{}.tar", folder_name);
        if verbose {
            println!("Tarball name: {:?}", tarball_name);
        }
        tarball_names_and_paths.insert(tarball_name, folder_path);
    }

    // print hashmap if verbose
    if verbose {
        println!("Tarball names and paths: {:?}", tarball_names_and_paths);
    }

    tarball_names_and_paths
}

/// Creates tarballs from the folder paths in the hashmap
fn tarballer(
    dry_run: bool,
    verbose: bool,
    remove: bool,
    names_and_paths: std::collections::HashMap<String, std::path::PathBuf>,
    current_dir: &Path,
) {
    // iterate over hashmap and create tarballs
    for (tarball_name, folder_path) in names_and_paths {
        let tarball_name = tarball_name.to_string();
        if verbose {
            println!("Tarball name: {:?}", tarball_name);
        }
        let folder_path = folder_path.to_str().unwrap();
        if verbose {
            println!("Folder path: {:?}", folder_path);
        }
        let tarball_path = format!("{}/{}", current_dir.to_str().unwrap(), tarball_name);
        if verbose {
            println!("Tarball path: {:?}", tarball_path);
        }
        let tarball_path = tarball_path.to_string();
        if verbose {
            println!("Tarball path as String: {:?}", tarball_path);
        }
        match dry_run {
            true => {
                println!("Dry run - would tarball folder: {:?}", folder_path);
                match remove {
                    true => {
                        println!("Dry run - would remove folder: {:?}", folder_path);
                    }
                    false => {
                        println!("Dry run - would NOT remove folder: {:?}", folder_path);
                    }
                }
            }

            false => {
                let file = File::create(tarball_path).unwrap();
                let mut archive = Builder::new(file);
                archive.append_dir_all(folder_path, folder_path).unwrap();
                if verbose {
                    println!("Tarball created: {:?}", tarball_name);
                }
                match remove {
                    true => {
                        if verbose {
                            println!("Removing folder: {:?}", folder_path);
                        }
                        std::fs::remove_dir_all(folder_path).unwrap();
                    }
                    false => {
                        if verbose {
                            println!("Not removing folder: {:?}", folder_path);
                        }
                    }
                }
            }
        }
    }
}
