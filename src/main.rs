use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::Write;
use pbr::ProgressBar;

fn main() {
    let root = Path::new(".");
    let mut errors = fs::File::create("errors.txt").unwrap();
    let mut log = fs::File::create("log.txt").unwrap();
    let mut count = 0;
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "inf" {
            count += 1;
        }
    }
    let mut progress = ProgressBar::new(count);
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "inf" {
            let output = Command::new("pnputil")
                .arg("/add-driver")
                .arg(entry.path())
                .output()
                .unwrap();
            if !output.status.success() {
                writeln!(errors, "{}: {:?}", entry.path().display(), output).unwrap();
            } else {
                writeln!(log, "{}: {:?}", entry.path().display(), output).unwrap();
            }
            progress.inc();
        }
    }
}