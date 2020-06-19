use reqwest::{Error};
use config;
use std::collections::HashMap;


mod api_readers;
mod groupme_api_structs;
mod file_handlers;
mod results_handlers;
mod stat_structs;
mod stats_engine;

fn main() -> Result<(), Error> {
	let mut settings = config::Config::default();
    settings
		.merge(config::File::with_name("Settings")).unwrap()
		.merge(config::Environment::with_prefix("APP")).unwrap();

	let setting_group_ids = settings.get_str("group_ids")
					.unwrap_or_default();

	let group_ids:Vec<&str> = setting_group_ids
									.split(",")
									.collect();
    
	let messages_output =  settings.get_str("output_folder")
										.unwrap_or_default();
	let results_output = settings.get_str("results_folder")
								 .unwrap_or_default();
								 
	file_handlers::raw_messages::clear_folder(&messages_output);
	file_handlers::raw_messages::clear_folder(&results_output);

    let api_key = settings.get_str("api_key")
							.unwrap_or_default();
    
                    
    for group_id in group_ids {
        let cur_group_id = group_id.parse::<u32>().unwrap();
        let res = api_readers::groupme_api_reader::run_reader(&api_key, cur_group_id, &messages_output);
        println!("{:?}", res);
    }

	let stats = &mut HashMap::new();
	println!("Starting running stats");
	let results = stats_engine::user_stat::run_stats(stats);
	println!("Saving results");
    results_handlers::result_handler::save_result(results);
	println!("Finished script");
    Ok(())
}


