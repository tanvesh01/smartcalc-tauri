extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;

fn main() {
    let test_data = r"$25/hour * 14 hours of work".to_string();
    initialize();
    let results = execute(&test_data, &"en".to_string());
    
    for result in results {
        match result {
            Ok((tokens, ast)) => {
                println!("{:?}", tokens);
                println!("{}", format_result(ast));
            },
            Err(error) => println!("Error : {}", error)
        };
    }
}

/*
cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ../www/
npm run start
*/
