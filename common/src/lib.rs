use std::fs;

pub fn load_vec_from_file(path: &str) -> anyhow::Result<Vec<String>> {
    let contents: String = fs::read_to_string(path)?.trim().into();
    Ok(contents.lines().map(|x| x.trim().into()).collect())
}

pub fn load_grid(path: &str) -> anyhow::Result<FlatGrid> {
    let contents = fs::read_to_string(path)?;
    let height = contents.lines().collect::<Vec<_>>().len();
    let width = contents.lines().collect::<Vec<_>>()[0].len();
    let cells = contents
        .lines()
        .flat_map(|line| line.as_bytes().to_vec())
        .collect();

    Ok(FlatGrid {
        width,
        height,
        cells,
    })
}

#[derive(Debug)]
pub struct FlatGrid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}
