use std::path::Path;
use walkdir::WalkDir;
use std::fs;

fn main() {
    let apath = "/media/pipi/taz/PicCD1".to_string();
    let mut count = 0;
    let mut removed = 0;
    let keep_vec = [
        
        "mpg", "MPG", "avi", "AVI", "mp4", "MP4", "mov", "MOV", "pdf", "PDF",
    ];

    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            count += 1;
            let fname = e.path().to_string_lossy().to_string();
            let ext_split = &fname.split(".").collect::<Vec<&str>>();
            let ext = ext_split.last().unwrap();
            print!("{} ", ext);
            if !keep_vec.contains(&ext) {
                println!("Removed: {}", &fname);
                std::fs::remove_file(fname.clone()).unwrap();
                removed += 1;
            }


            
        }
    }
    remove_empty_dirs(Path::new(&apath.clone())).unwrap();
    println!("Start count: {}\nFiles removed: {}", count, removed.clone());
}

fn remove_empty_dirs(root_dir: &Path) -> std::io::Result<()>{
    for entry in root_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.read_dir()?.next().is_none() {
                fs::remove_dir_all(path)?;
            }
        }
    }
    
    Ok(())
}

