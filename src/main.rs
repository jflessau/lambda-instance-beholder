use anyhow::{Error, Result};
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, error, info, warn};
use reqwest::{cookie::Jar, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, sync::Arc, thread, time};

fn main() {
    dotenv().ok();
    let mut builder = Builder::from_default_env();
    builder.init();

    let instance_types_of_interest: Vec<String> = env::var("INSTANCE_TYPES_OF_INTEREST")
        .unwrap_or_else(|_| "".to_string())
        .split(',')
        .map(|v| v.trim().to_string())
        .collect::<Vec<String>>();

    if instance_types_of_interest.is_empty() {
        warn!("no instance types of interest provided");
        return;
    }

    let mut instance_types: HashMap<String, bool> = HashMap::new();
    let mut print_instance_type_list = true;

    loop {
        match refresh_instance_types(
            &mut instance_types,
            &instance_types_of_interest,
            print_instance_type_list,
        ) {
            Err(err) => {
                error!("failed to refresh instance types, error: {}", err);
                break;
            }
            Ok(done) => {
                debug!("refresh_instance_types completed successfully");
                if done {
                    break;
                }
            }
        }

        let grace_period = time::Duration::from_secs(5 * 60);
        info!("will refresh in {} seconds...", grace_period.as_secs());
        thread::sleep(grace_period);

        print_instance_type_list = false;
    }
}

fn refresh_instance_types(
    current_instance_types: &mut HashMap<String, bool>,
    instance_types_of_interest: &[String],
    print_instance_type_list: bool,
) -> Result<bool, Error> {
    debug!("current_instance_types: {:?}", current_instance_types);

    let mut updates = Vec::new();
    let cookie_str = env::var("SESSION_ID").expect("SESSION_ID not set");

    let url = "https://cloud.lambdalabs.com".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(
        &format!("sessionid={}; Domain=lambdalabs.com", cookie_str),
        &url,
    );
    let jar = Arc::new(jar);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar)
        .build()?;

    let response = client
        .get("https://cloud.lambdalabs.com/api/v1/instance-types")
        .send()?;

    let response_status = response.status();
    let response_text = response.text()?;

    if response_status == reqwest::StatusCode::from_u16(401)? {
        anyhow::bail!("unauthorized: check your SESSION_ID");
    } else if response_status != reqwest::StatusCode::from_u16(200)? {
        anyhow::bail!("calling api fails with status code {}", response_status);
    }

    let response: Response = serde_json::from_str(&response_text)?;

    debug!("response: {:?}", response);

    if let Value::Object(ref new_instance_types) = response.data {
        if print_instance_type_list {
            info!(
                "all instance types: {:?}",
                new_instance_types
                    .iter()
                    .map(|(k, _)| k.to_string())
                    .collect::<Vec<String>>()
            );
            info!(
                "instance types of interest: {:?}",
                instance_types_of_interest
            );
        }

        new_instance_types.iter().for_each(|(k, new_available)| {
            let new_available = match new_available.get("regions_with_capacity_available") {
                Some(Value::Array(a)) => !a.is_empty(),
                _ => false,
            };

            if let Some(old_available) = current_instance_types.get_mut(k) {
                if instance_types_of_interest.contains(k) && new_available != *old_available {
                    debug!("add update for: {}", k);
                    updates.push((k.to_string(), new_available));
                }
                *old_available = new_available
            } else {
                if instance_types_of_interest.contains(k) {
                    debug!("add update for: {}", k);
                    updates.push((k.to_string(), new_available));
                }
                current_instance_types.insert(k.to_string(), new_available);
            }
        });
    }

    let mut done = false;

    for update in &updates {
        info!(
            "{} {} is {}",
            if update.1 { "✅" } else { "🚫" },
            update.0,
            if update.1 {
                "now available!"
            } else {
                "not available"
            },
        );
        if update.1 {
            done = true;
        }
    }

    Ok(done)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub data: Value,
}
