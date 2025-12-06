use std::fs;

pub fn load_vec_from_file(path: &str) -> anyhow::Result<Vec<String>> {
    let contents: String = fs::read_to_string(path)?.trim().into();
    Ok(contents.lines().map(|x| x.trim().into()).collect())
}
