fn main() -> PolarsResult<()> {
    let nomes = Series::new("Nome", &["Alice", "Bob", "Carlos"]);
    let idades = Series::new("Idade", &[25, 30, 35]);

    let df = DataFrame::new(vec![nomes, idades])?;
    println!("{}", df);

    Ok(())
}