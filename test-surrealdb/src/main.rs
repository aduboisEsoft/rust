use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::opt::auth::Scope;

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;

    // Create a root user
    // db.signup(Scope {
    //     namespace: "test",
    //     database: "test",
    //     scope:"account",
    //     params: Credentials {
    //         email:"usertesta",
    //         pass:"usertesta"
    //     },
    // })
    // .await?;

    // Signin as a namespace, database, or root user
    db.signin(Scope {
        namespace: "test",
        database: "test",
        scope:"account",
        params: Credentials {
            email:"usertesta",
            pass:"usertesta"
        },
    })
    .await?;

    // // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // // Create a new person with a random id
    let created: Record = db
        .create("person")
        .content(Person {
            title: "CTO",
            name: Name {
                first: "Arnaud",
                last: "Dubois",
            },
            marketing: false,
        })
    .await?;
    dbg!(created);

    // // Update a person record with a specific id
    // let updated: Record = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // // Select all people records
    // let people: Vec<Record> = db.select("person").await?;
    // dbg!(people);

    // // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);

    Ok(())
}