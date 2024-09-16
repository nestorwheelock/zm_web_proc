use std::env;
use std::fs;
use std::process::{Command, exit};
use std::path::{Path};

fn unzip_file(zip_file: &str) -> Result<String, std::io::Error> {
    let output = Command::new("unzip")
        .arg(zip_file)
        .output()?;
    
    if !output.status.success() {
        eprintln!("Failed to unzip the file: {}", zip_file);
        exit(1);
    }

    // Extract directory name from zip file name
    let dir_name = zip_file.replace(".zip", "");
    Ok(dir_name)
}

fn run_zm_vidsort(dir_name: &str) -> Result<(), std::io::Error> {
    let status = Command::new("zm_vidsort")
        .current_dir(dir_name)
        .status()?;
    
    if !status.success() {
        eprintln!("zm_vidsort command failed");
        exit(1);
    }

    Ok(())
}

fn run_mp4_to_webm(dir_name: &str) -> Result<(), std::io::Error> {
    println!("Running mp4_to_webm in directory: {}", dir_name);
    
    let status = Command::new("mp4_to_webm")
        .arg(dir_name)
        .status()?;
    
    if !status.success() {
        eprintln!("mp4_to_webm command failed with status: {}", status);
        exit(1);
    } else {
        println!("mp4_to_webm ran successfully.");
    }

    Ok(())
}

fn move_webm_files(src_dir: &str, dest_dir: &str) -> Result<(), std::io::Error> {
    // Ensure the destination "videos" directory exists
    let videos_dir = Path::new(dest_dir);
    if !videos_dir.exists() {
        fs::create_dir_all(&videos_dir)?;
    }

    let entries = fs::read_dir(src_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "webm" {
            let new_location = videos_dir.join(path.file_name().unwrap());
            fs::rename(&path, &new_location)?;
            println!("Moved file: {:?} to {:?}", path, new_location);
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: zm_web_proc <zm_export_zip_file>");
        exit(1);
    }

    let zip_file = &args[1];
    let dir_name = unzip_file(zip_file)?;

    run_zm_vidsort(&dir_name)?;

    run_mp4_to_webm(&dir_name)?;  // Run the video conversion process

    // Move .webm files from the unzipped directory to the "videos" directory in the current working directory
    move_webm_files(&dir_name, "videos")?;

    println!("Processing complete.");

    Ok(())
}

