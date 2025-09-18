mod errs;
use errs::fetch;

#[tokio::main]
async fn main() {
    let animals = match fetch("https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/foo.json").await {
        Ok(animals) => {
            println!("{animals:#?}");
            // Will return parsed JSON as Vec<AnimalData> type.
            animals
        }
        Err(err) => {
            // Will yelling.
            println!("No animals!: {:?}", err);
            
            // Will return empty vector animals.
            Vec::from([])
        }
    };

    println!("{animals:#?}");
}
