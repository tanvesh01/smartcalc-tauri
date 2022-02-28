/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use core::ops::Deref;

use alloc::collections::btree_map::BTreeMap;

use crate::config::SmartCalcConfig;
use crate::types::TimeOffset;
use crate::worker::tools::get_text;
use crate::worker::tools::get_time;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};
use crate::tools::parse_timezone;

const TIMEZONE_PATTERN: &'static str = "(?P<timezone>(?P<timezone_2>GMT(?P<timezone_type>[+-]?)(?P<timezone_hour>[0-1]?[0-9]):?(?P<timezone_minute>[0-5][0-9])?)?(?P<timezone_1>[A-Z]{1,4})?)";

pub fn time_for_location(_: &SmartCalcConfig, _: &Tokinizer, atoms: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if let Some(TokenType::Text(_location)) = &atoms.get("location").unwrap().token_type.borrow().deref()  {
        /*{
            let json_data = fs::read_to_string("/Users/erhanbaris/ClionProjects/smartcalculator/smartcalc/src/json/city_informations.json").expect("{}");
            let json_value: Result<Value> = from_str(&json_data);

            return match json_value {
                Ok(data) => {
                    for item in data.as_array().unwrap() {
                        if let Value::String(city) = item.get("city_ascii").unwrap() {

                            if city.to_lowercase() == location.to_lowercase() {
                                let timezone = item.get("timezone").unwrap().as_str().unwrap();
                                let tz: Tz = match timezone.parse() {
                                    Ok(v) => v,
                                    Err(_) => return Err("Time not found".to_string())
                                };
                                return Ok(TokenType::Time(Utc::now().with_timezone(&tz).naive_local().time()));
                            }
                        }
                    }

                    Err("Time not found".to_string())
                },
                Err(error) => {
                    //println!("{}", error);
                    Err("Internal error".to_string())
                }
            };
        }*/
    }

    Err("Location not found".to_string())
}

pub fn time_with_timezone(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("timezone") {
        
        let (time, _) = get_time("time", &fields).unwrap();
        let timezone = get_text("timezone", &fields).unwrap();
        
        let timezone = match regex::Regex::new(&TIMEZONE_PATTERN) {
            Ok(re) => match re.captures(&timezone[..]) {
                Some(capture) => parse_timezone(config, &capture),
                None => None
            },
            _ => None
        };
        
        let (timezone, offset) = match timezone {
            Some((timezone, offset)) => (timezone, offset),
            None => return Err("Timezone not found".to_string())
        };

        return Ok(TokenType::Time(time, TimeOffset { 
            name: timezone.to_uppercase(),
            offset
         }));
    }
    Err("Timezone or time informations not found".to_string())
}
