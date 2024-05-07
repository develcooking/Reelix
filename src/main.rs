use mysql::*;
use mysql::prelude::*;
use std::io;

fn main() {
    let database_url = "mysql://theuser:12345@141.13.222.38:3306/thebanking1";

    let pool = Pool::new(database_url)
        .expect("Failed to create a connection pool");

    let mut conn = pool.get_conn()
        .expect("Failed to get a connection from the pool");

    println!("Bitte geben Sie den zu löschenden Typen ein:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Fehler beim Einlesen der Eingabe");
    let typen = input.trim();

    conn.exec_drop(
        r"DELETE FROM Typtabelle WHERE Typen = ?",
        (&typen,)
    ).expect("Fehler beim Löschen der Daten");
    
    println!("Daten erfolgreich gelöscht!");
}
