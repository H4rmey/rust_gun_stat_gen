use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;
use std::convert::TryInto;
use std::ops::{Add, Sub, Mul, Div};

const ATTRIBUTE_COUNT: usize = 10;
const SEED_LENGTH: usize = 32;

#[warn(dead_code)]
pub struct GunStats
{
    pub seed: String,
    pub rarity: RARITY,
    pub stats: Vec<f64>
}

pub enum ATTRIBUTES
{
    ReloadTime = 0,
    TravelSpeed = 1,
    Damage = 2,
    MagazineSize = 3,
    Spread = 4,
    Firerate = 5,
    BulletsPerShot = 6,
    BulletSize = 7,
    BulletShape = 8,
    WalkSpeed = 9
}

#[derive(Clone, Copy)]
pub enum RARITY
{
    White = 0,
    Green = 1,
    Blue = 2,
    Orange = 3
}

impl GunStats
{
    pub fn new() -> GunStats
    {
        GunStats 
        {         
            seed: String::new(),
            rarity: RARITY::Orange,
            stats: vec![1.0; ATTRIBUTE_COUNT],
        }
    }

    pub fn generate_with_seed(&mut self, seed: &str, ranges: [[f64; 2]; ATTRIBUTE_COUNT])
    {
        let temp_seed: Vec<u8>          = seed.as_bytes().iter().cloned().collect();

        let empty_cell_count: usize     = SEED_LENGTH-temp_seed.len();

        let empty_seed: Vec<u8>         = vec![0; empty_cell_count];

        let converted_seed: &[u8]       = &[temp_seed, empty_seed].concat();
        
        let mut rng: StdRng = SeedableRng::from_seed(converted_seed.try_into().expect("slice with incorrect length")); 

        self.seed = seed.to_owned();

        for i in 0..self.stats.len()
        {
            let old_value: f64 = rng.gen::<f64>();
            self.stats[i] = GunStats::gen_with_range::<f64>(old_value, ranges[i][0], ranges[i][1], 0.0, 1.0);
        }

        self.rarity = match GunStats::gen_with_range::<f64>(
                                    rng.gen::<f64>(), 
                                    RARITY::White as usize as f64, 
                                    RARITY::Orange  as usize as f64 + 1.0, 
                                    0.0, 
                                    1.0
                                ) as usize
                            {
                                0 => RARITY::White,
                                1 => RARITY::Green,
                                2 => RARITY::Blue,
                                3 => RARITY::Orange,    
                                _ => panic!("gvd was deze?!?!"),
                            };

        for _i in 0..self.rarity as usize
        {
            let attr: usize = GunStats::gen_with_range::<f64>(rng.gen::<f64>(), 0.0, 10.0, 0.0, 1.0) as usize;
            println!("{:?}", attr);
            self.stats[attr] *= 0.0;
        }
    }

    fn gen_with_range<T>(old: T, ori_low: T, ori_high: T, bound_low: T, bound_high: T) -> T 
    where 
    T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Copy,
    {
        let r: T =  (old - bound_low) / (bound_high - bound_low) * (ori_high - ori_low) + ori_low;

        r
    }
}