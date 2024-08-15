#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub RAM: Vec<RAMs>
}

#[derive(Debug, Deserialize)]
pub struct RAMs {
    pub Order: usize,
    pub Capacity: u64,
    pub Technology: String,
    pub CPUReq: usize,
    pub OSReq: usize
}