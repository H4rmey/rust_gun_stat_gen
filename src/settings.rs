
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SETTINGS_PATH: &str       = "settings.json";
const ATTRIBUTE_COUNT: usize    = 10;

pub struct Settings
{
    pub settings_json: serde_json::Value,
    pub attributes: [String; ATTRIBUTE_COUNT],
    pub ranges: [[f64; 2]; ATTRIBUTE_COUNT],
}

impl Settings
{
    pub fn new() -> Settings
    {
        let mut s = Settings
        {
            settings_json: Settings::get_settings(),
            attributes: Default::default(),
            ranges: [[1.0; 2]; ATTRIBUTE_COUNT],
        };
        
        for i in 0..s.settings_json["attributes"].as_array().unwrap().len()
        {
            s.attributes[i] = s.settings_json["attributes"]
                                .as_array().unwrap()[i]
                                .as_str().unwrap().to_string();
        }

        s.ranges = s.get_ranges();

        s
    }

    fn get_ranges(&self) -> [[f64; 2]; ATTRIBUTE_COUNT]
    {
        let attr = &self.attributes;
        let mut v_out: [[f64; 2]; ATTRIBUTE_COUNT] = [[0.0; 2]; ATTRIBUTE_COUNT];
        
        // println!("{}", attr[0]);
        for k in 0..ATTRIBUTE_COUNT
        {
            let i= self.settings_json["ranges"][attr[k].as_str()].as_array().unwrap();
            // println!("{} - {}", i[0], i[1]);

            for p in 0..2
            {
                v_out[k][p] = i[p].as_f64().unwrap() as f64;
            }
        }

        v_out
    }

    fn get_settings() -> serde_json::Value
    { 
        // Create a path to the desired file
        let path = Path::new(SETTINGS_PATH);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut settings = String::new();
        match file.read_to_string(&mut settings) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}\n", display, settings),
        }
        
        serde_json::from_str(&settings[..]).expect("json error")
    }
}

