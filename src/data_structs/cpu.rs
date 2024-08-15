#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub AmountOfCPUs: u32,
    pub CPUs: Vec<CPU>,
}

#[derive(Debug, Deserialize)]
pub struct CPU {
    pub Order: usize,
    pub Name: String,
    pub Date_Available: String,
    pub Description: String,
    pub Clock_Mhz: u32,
    pub Lithography: u32,
    pub Cores: u32,
    pub Threads: u32,
    pub L1Cache: u32,
    pub L2Cache: u32,
    pub L3Cache: u32,
    pub TDP: u32,
    pub Architecture: String,
}