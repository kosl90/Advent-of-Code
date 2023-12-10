use std::ffi::OsStr;

pub fn read_file_in_examples<S, P>(root: &S, filename: P) -> std::io::Result<std::fs::File>
where
    S: AsRef<OsStr> + ?Sized,
    P: AsRef<std::path::Path>,
{
    let p = std::path::Path::new(root).join("examples").join(filename);
    // dbg!("read file in examples: {:?}", &p);
    std::fs::OpenOptions::new().read(true).open(p)
}
