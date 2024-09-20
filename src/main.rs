use clap::Parser;
use std::path::Path;
use tar::Builder;
// use std::io::prelude::*;
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author = "Maxwell Rupp", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }

    // print current directory
    let current_dir = Path::new("."); // "/home/gig/local_repos/recursive-tarballs/testing/test-folder"; // std::env::current_dir().unwrap();
    // println!("Current directory: {:?}", current_dir);

    // // list files in current directory
    // let paths = std::fs::read_dir(current_dir).unwrap();
    // for path in paths {
    //     println!("Name: {:?}", path.unwrap().path());
    // }

    // start vec of folder paths
    let mut folder_paths = Vec::new();

    // filter paths to only include folders
    let paths = std::fs::read_dir(current_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            folder_paths.push(path);
        }
    }

    // pretty print the folder paths vec
    // println!("Folder paths: {:?}", folder_paths);

    // start new hashmap for tarball names
    let mut tarball_names_and_paths = std::collections::HashMap::new();

    // iterate over folder paths and add to hashmap with {folderName}.tar as key and path as value
    for folder_path in folder_paths {
        let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
        // println!("Folder name: {:?}", folder_name);
        let tarball_name = format!("{}.tar", folder_name);
        // println!("Tarball name: {:?}", tarball_name);
        tarball_names_and_paths.insert(tarball_name, folder_path);
    }

    // pretty print the hashmap
    // println!("Tarball names and paths: {:?}", tarball_names_and_paths);

    // iterate over hashmap and create tarballs
    for (tarball_name, folder_path) in tarball_names_and_paths {
        // println!("Tarball name: {:?}", tarball_name);
        // println!("Folder path: {:?}", folder_path);
        let tarball_name = tarball_name.to_string();
        let folder_path = folder_path.to_str().unwrap();
        let tarball_path = format!("{}/{}", current_dir.to_str().unwrap(), tarball_name);
        // println!("Tarball path: {:?}", tarball_path);
        let tarball_path = tarball_path.to_string();
        // run 'tar -cvzf {tarball_name} {folder_path}'
        let file = File::create(tarball_path).unwrap();
        let mut archive = Builder::new(file);
        archive.append_dir_all(folder_path, folder_path).unwrap();

        if args.remove == Some("remove".to_string()) {
            // delete archived folder at folder_path
            std::fs::remove_dir_all(folder_path).unwrap();
        };
    }
}
