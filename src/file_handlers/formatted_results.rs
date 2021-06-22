use config;
use std::fs::File;
use std::io::Write;


pub fn save_as_text(results: Vec<Vec<String>>){
	let mut settings = config::Config::default();
    settings
		.merge(config::File::with_name("Settings")).unwrap()
		.merge(config::Environment::with_prefix("APP")).unwrap();

	let filename =  settings.get_str("results_folder").unwrap_or_default() + "/results.txt";

    let mut f = File::create(filename).expect("Unable to open file");
    for cur_user in results {
        cur_user.iter().for_each(|res_line| f.write_all(res_line.as_bytes()).expect("Unable to write data"));
    }
}
