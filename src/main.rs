use std::fs;
use walkdir::WalkDir;

fn main() {
    let img_path = "/media/pipi/0123-4567/Images".to_string();
    let _rm_unwanted = rm_unwanted_files(img_path.clone());
    let _rm_by_ext = rm_by_extension(img_path.clone());
}

pub fn rm_unwanted_files(apath: String) -> i32 {
    let to_remove_list = [
        "0611160110.jpg",
        "0821161127a.jpg",
        "0821161127.jpg",
        "0821161128a.jpg",
        "0821161128.jpg ",
        "0821162314.jpg",
        "0822161108a.jpg",
        "0822161108.jpg",
        "0906141903a.jpg",
        "20150327_200735.jpg",
        "DSCN1120.JPG",
        "h150.jpg",
        "h151.jpg",
        "h152.jpg",
        "h153.jpg",
        "h154.jpg",
        "h155.jpg",
        "h156.jpg",
        "h157.jpg",
        "h158.jpg",
        "h159.jpg",
        "h15.jpg",
        "h160.jpg",
        "h161.jpg",
        "h162.jpg",
        "h163.jpg",
        "h164.jpg",
        "h165.jpg",
        "h166.jpg",
        "h167.jpg",
        "h168.jpg",
        "h169.jpg",
        "h16.jpg",
        "h170.jpg",
        "h171.jpg",
        "h172.jpg",
        "h173.jpg",
        "h17.jpg",
        "h18.jpg",
        "h19.jpg",
        "my_touch_phone_023.gif",
        "mytouchphone023.gif",
        "normallit_16_h.png",
        "normallit_16.png",
        "PART_1454822819828_20160206_131602.jpg",
        "phone_023.gif",
        "phone023.gif",
        "phone_270.gif",
        "phone270.gif",
        "phone276.jpg",
        "P-009.tar",
    ];

    let mut idx = 0;
    let mut rmcount = 0;
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            // let tm = fname.split("/").collect::<Vec<&str>>();
            // let filename = tm.last().unwrap();
            if fname.contains("System") {
                rmcount += 1;
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
            } else if fname.contains("python3-openid") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("torando") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("DO.NOT.DELETE") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("jqm-pagination-master") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if fname.contains("pussy") {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else if to_remove_list.contains(&fname.as_str()) {
                rmcount += 1;
                std::fs::remove_file(fname.clone()).unwrap();
            } else {
                println!("fuck")
            }
        }
    }
    println!("Start count: {}\nFiles removed: {}", idx, rmcount.clone());

    rmcount
}
pub fn rm_by_extension(apath: String) -> i32 {
    let rm_list = [
        "mp3",
        "MP3",
        "wav",
        "WAV",
        "yaml",
        "py",
        "sql",
        "in",
        "rst",
        "sh",
        "cfg",
        "c",
        "csv",
        "mo",
        "po",
        "crt",
        "ini",
        "m4p",
        "m4a",
        "key",
        "htm",
        "txt",
        "ps1",
        "jar",
        "dat",
        "3gp",
        "nfo",
        "m3u",
        "jpgblk",
        "THM",
        "torrent",
        "info",
        "epp",
        "db",
        "mix",
        "xml",
        "doc",
        "itl",
        "ssf",
        "bak",
        "ctl",
        "lnk",
        " SF",
        "exe",
        "thm",
        "docx",
        "js",
        "css",
        "html",
        "colorstarmutedjpg",
        "redsheartsswirljpg",
        "redyucaflowerjpg",
        "wrapwoodjpg",
    ];
    let mut count = 0;
    let mut rmcount = 0;

    for e in WalkDir::new(&apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            count += 1;
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();

            let ext = parts.last().unwrap();
            if rm_list.contains(&ext) {
                println!("Removed: {}", &fname);
                fs::remove_file(&fname).unwrap();
                rmcount += 1;
            };
        };
    }
    println!("rm by ext start count: {}\nFiles removed: {}", count, rmcount.clone());

    rmcount.clone()
}