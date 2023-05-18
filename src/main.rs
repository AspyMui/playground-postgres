use playground_postgres::db;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = db::new()?;

    let bob = db::create_person(&mut client, String::from("Bob"))?;
    println!("{:?}", bob);

    let peep = db::get_person(&mut client, 1)?;
    println!("{:?}", peep);

    let peeps = db::get_people(&mut client)?;

    for peep in peeps {
        println!("{:?}", peep);
    }

    Ok(())
}
