use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
struct Typ {
    typ: String,
}

fn main() {
    let database_url = "mysql://theuser:12345@141.13.222.38:3306/thebanking1";

    let pool = Pool::new(database_url)
        .expect("Failed to create a connection pool");

    let mut conn = pool.get_conn()
        .expect("Failed to get a connection from the pool");

    let types: Vec<Typ> = conn.query_map(
        r"SELECT * FROM Typtabelle ORDER BY Typen ASC;",
        |typ| {
            Typ { typ }
        },
    ).expect("Failed to fetch data");


    for typ in types {
        println!("{}", typ.typ);
    }
}
