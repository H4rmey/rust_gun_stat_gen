pub mod gunstats;
pub mod settings;
use gunstats::*;
use settings::*;

fn main() 
{
    let seed: [&str; 10] = [
        "F2EZwLOeq4wMJgd4r9wX",
        "KSAXdzBmlq1NsQqeMleu",
        "4kDcXMGiWooytD4JOFfA",
        "V5f239y4EaPUQfRU3I5v",
        "LY1WawK38jkRtOzzjEOy",
        "jkGC7PtQTI9CGklfRN6t",
        "nxfcEyA02m9Ce0WecaZS",
        "jQtHCiPd0uQEimGVF0cK",
        "h4RbKbGnvh8iEu7dGP4B",
        "P3eRzKiTh9niMfTeLNjh"
    ];

    let settings: Settings = Settings::new();

    let mut gun: Vec<GunStats> = Vec::with_capacity(10);
    for _ in 0..10 
    {
        gun.push(GunStats::new());
    }

    for i in 0..10
    {
        gun[i].generate_with_seed(seed[i], settings.ranges);

        println!("----------{}----------", gun[i].seed);   
        for k in 0..10
        {
            println!("{}:   {}", settings.attributes[k], gun[i].stats[k]);   
        }
    }
}
