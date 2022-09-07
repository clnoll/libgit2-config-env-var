use std::env;

fn main() {
    match env::current_dir() {
        Ok(dir) => {
            match git2::Repository::discover(dir) {
                Ok(repo) => match repo.config() {
                    Ok(c) => match c.get_entry("myconfigsection.custom") {
                        Ok(entry) => match entry.value() {
                            Some(value) => println!("{}", value),
                            None => println!("No value XXXX"),
                        },
                        Err(err) => {
                            println!("Error: {}", err)
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err)
                    }
                },
                Err(err) => {
                    // git2::Config::open_default().ok()
                    println!("Error: {}", err)
                }
            };
        }
        Err(err) => {
            println!("Error: {}", err)
        }
    };
}
