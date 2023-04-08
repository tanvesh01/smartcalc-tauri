/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

extern crate smartcalc_tauri;

use std::borrow::{Borrow, BorrowMut};
use std::ops::Deref;

use smartcalc_tauri::money::MoneyItem;
use smartcalc_tauri::*;

fn main() {
    use chrono::{Local, TimeZone};
    use chrono_tz::OffsetName;
    use chrono_tz::Tz;
    use num_format::SystemLocale;
    let locale = SystemLocale::default().unwrap();
    let timezone = match localzone::get_local_zone() {
        Some(tz) => match tz.parse::<Tz>() {
            Ok(tz) => {
                let date_time = Local::today().naive_local();
                tz.offset_from_utc_date(&date_time)
                    .abbreviation()
                    .to_string()
            }
            Err(_) => "UTC".to_string(),
        },
        None => "UTC".to_string(),
    };

    let test_data = r"today - may 8".to_string();
    let mut app = SmartCalc::default();

    //app.add_rule("en".to_string(), vec!["{TEXT:soyad:erhan}".to_string()], test1 as RuleFunction);
    app.set_decimal_seperator(locale.decimal().to_string());
    app.set_thousand_separator(locale.separator().to_string());
    app.set_timezone(timezone).unwrap();

    let language = "en".to_string();
    let results = app.execute(language, test_data);
    println!("Results: {:?}", results);
    println!("0-=================================0");
    // let tauri_result;
    for result in results.lines.iter() {
        match result {
            Some(result) => match &result.result {
                Ok(output) => {
                    println!("{:?}", output);
                    // for tokens in result.calculated_tokens.iter() {
                    //     // get value from refcell token_type
                    //     let some = tokens.token_type.borrow();
                    //     // .deref();
                    //     println!("as token => {:?}", some.as_ref().unwrap().type_name());
                    //     println!("as REFFFFF => {:?}", some.as_ref().unwrap().to_string());

                    //     // if tokens
                    //     //     .token_type
                    //     //     .borrow()
                    //     //     .as_ref()
                    //     //     .unwrap()
                    //     //     .borrow_mut()
                    //     //     .type_name()
                    //     //     == "MONEY"
                    //     // {
                    //     //     println!("CHA CHING: {:?}", tokens);
                    //     //     println!(
                    //     //         "{:?} ",
                    //     //         tokens.token_type.borrow_mut().as_ref().expect("Error")
                    //     //     )
                    //     // }
                    //     // println!(
                    //     //     "{:?}",
                    //     //     tokens
                    //     //         .token_type
                    //     //         .borrow_mut()
                    //     //         .as_ref()
                    //     //         .expect("Error")
                    //     //         .type_name()
                    //     // );
                    // }
                }
                Err(error) => println!("Error : {}", error),
            },
            None => println!("No query"),
        }
    }
}
