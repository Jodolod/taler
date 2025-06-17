fn main() {
    let _ = std::fs::remove_file("dev.db");
    let mut paths = std::fs::read_dir("./migrations/")
        .unwrap()
        .collect::<std::io::Result<Vec<_>>>()
        .unwrap();
    paths.sort_by_key(|p| p.path());

    for file in paths {
        let stdin = std::fs::File::open(file.path()).unwrap();
        let status = std::process::Command::new("sqlite3")
            .arg("dev.db")
            .stdin(stdin)
            .status()
            .unwrap();
        assert!(status.success());

        println!("cargo:rerun-if-changed={}", file.path().display());
    }
}
