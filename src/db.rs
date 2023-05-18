use postgres::{Client, NoTls};
use postgres_types::{FromSql, ToSql};

#[derive(Debug, ToSql, FromSql)]
pub struct Person {
    person_id: i32,
    name: String,
}

impl Person {
    fn new(person_id: i32, name: String) -> Person {
        Person { person_id, name }
    }
}

pub fn new() -> Result<Client, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let db_pass = std::env::var("DB_PASS")?;
    let mut client = Client::connect(
        std::format!("host=localhost user=postgres password={db_pass} dbname=test").as_str(),
        NoTls,
    )?;

    client.batch_execute(
        r#"
    CREATE TABLE IF NOT EXISTS person (
        person_id   SERIAL  PRIMARY KEY,
        name        TEXT    NOT NULL
    )
    "#,
    )?;

    Ok(client)
}

pub fn get_person(client: &mut Client, person_id: i32) -> Result<Person, postgres::Error> {
    let row = client.query_one(
        "SELECT * FROM  person WHERE person_id=$1 LIMIT 1",
        &[&person_id],
    )?;

    Ok(Person::new(person_id, row.get("name")))
}

pub fn create_person(client: &mut Client, name: String) -> Result<Person, postgres::Error> {
    let count = client.execute("INSERT INTO person(name) VALUES ($1)", &[&name])?;
    println!("How many? {}", count);
    let row = client.query_one(
        "SELECT * FROM person WHERE name=$1 ORDER BY person_id LIMIT 1",
        &[&name],
    )?;

    Ok(Person::new(row.get("person_id"), name))
}

pub fn get_people(client: &mut Client) -> Result<Vec<Person>, postgres::Error> {
    let rows = client.query("SELECT * FROM person", &[])?;
    let mut people: Vec<Person> = Vec::new();

    for row in rows {
        people.push(Person::new(row.get("person_id"), row.get("name")));
    }

    Ok(people)
}
