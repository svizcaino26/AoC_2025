use std::fs;

pub fn load_vec_from_file(path: &str) -> anyhow::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(|x| x.into()).collect())
}
