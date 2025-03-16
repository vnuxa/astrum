use time::OffsetDateTime;
use std::{fs::{self, create_dir, exists, read, read_dir, read_to_string, DirBuilder, DirEntry, File, OpenOptions}, io::Write, path::{Path, PathBuf}, thread::yield_now, time::SystemTime};
use std::process::Command;


fn read_dir_and_compare(time: SystemTime, dir: &Path) -> bool {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                if read_dir_and_compare(time, &path) == true {
                    return true;
                }
            } else if entry.metadata().unwrap().modified().unwrap() > time{
                return true;
            }
        }
    }
    false
}
fn main() {

    let mut needs_update = true;

    println!("cargo::rerun-if-changed=src/lua_library");
    println!("cargo::rerun-if-changed=docs/src");
    if exists("./docs/last_updated.txt").unwrap() {
        // let file = read_to_string("./docs/last_updated.txt").unwrap();
        let file = File::open("./docs/last_updated.txt").unwrap();
        let last_modified = file.metadata().unwrap().modified().unwrap();

        needs_update = read_dir_and_compare(last_modified, &fs::canonicalize("./src/lua_library/").unwrap());
    } else {
        needs_update = true;
    }

    if needs_update {
        let _ = Command::new("lua-language-server")
            .current_dir(fs::canonicalize("./").unwrap())
            .args([
                "--configpath",
                "luadoc.json",
                "--doc",
                "./src/lua_library/astrum/types/",
                "--doc_out_path",
                "./docs/src/doc_out"
            ]).output().unwrap();

        let current_date: String = OffsetDateTime::now_utc().to_string();
        let _ =  std::fs::write("./docs/last_updated.txt", &current_date);
        // let _ =


        // NOTE: should probably make it so that it runs the exe that zig makes when using zig bu
        // ild rtun thater htan running this command every time

        let _ = Command::new("zig")
            .current_dir(fs::canonicalize("./docs").unwrap())
            .args([
                "build",
                "run"
            ]).output().unwrap();

        // let _ = Command::new("mdbook")
        //     .current_dir(fs::canonicalize("./docs").unwrap())
        //     .arg("build").output().unwrap();

// [Last auto-generated: 2025-02-28]()

// ---
        // println!("got output:");
        // std::io::stdout().write_all(&command.stdout).unwrap();
        //
        // println!("got error:");
        // std::io::stdout().write_all(&command.stderr).unwrap();

    }

}
