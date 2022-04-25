use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);

    //expect args to be provided
    let key = args.next().expect("No key provided!");
    let value = args.next().expect("No value provided!");

    //create db from file
    let mut database = Database::new().expect("Database::new() crashed");

    //insert new (key,value)
    database.insert(key, value);

    //flush db to file
    //database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        let mut map = HashMap::new();

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_string(), value.to_string());
        }
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> Result<(), std::io::Error> {
        flush_database_to_file(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = flush_database_to_file(self);
    }
}

fn flush_database_to_file(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();

    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
