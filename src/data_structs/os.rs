#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub AmountOfOSes: u32,
    pub OSes: Vec<OSes>
}

#[derive(Debug, Deserialize)]
pub struct OSes {
    pub Order: usize,
    pub Name: String,
    pub CPUReq: usize,
    pub RAMReq: usize
}