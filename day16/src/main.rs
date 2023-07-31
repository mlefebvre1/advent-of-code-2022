mod defs;

fn main() -> anyhow::Result<()> {
    println!("First part : {}", first()?);
    println!("Second part : {}", second()?);
    Ok(())
}

#[allow(non_upper_case_globals)]
fn first() -> anyhow::Result<String> {
    let data = std::fs::read_to_string("day16/data/day16.txt")?;
    let solve = defs::Solve::new(&data);
    let ans = solve.solve_part1();
    Ok(ans.to_string())
}

fn second() -> anyhow::Result<String> {
    let data = std::fs::read_to_string("day16/data/day16.txt")?;
    let solve = defs::Solve::new(&data);
    let ans = solve.solve_part2();
    Ok(ans.to_string())
}
