mod defs;

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(non_upper_case_globals)]
fn first() -> anyhow::Result<String> {
    let data = std::fs::read_to_string("day16/data/day16_easy.txt")?;
    let nodes = defs::Nodes::new(&data);
    nodes.solve();
    Ok("".to_string())
}

fn second() -> anyhow::Result<String> {
    Ok("".to_string())
}
