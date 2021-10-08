use accessor::Setters;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use log::{error, info, warn};
use futures::future::join_all;
use std::sync::{Mutex, Arc};
use std::ops::Deref;
use std::borrow::Borrow;
use std::mem;
use std::convert::TryInto;
use std::hash::Hash;

static PARALLEL_EXECUTIONS: usize = 100; // TODO add a conf for this
static WARP_CONF_PATH: &str = "./npc/scripts_warps.conf";

#[derive(Setters, Clone)]
pub struct Warp {
    #[set]
    pub map_name: String,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    #[set]
    pub x_size: u16,
    #[set]
    pub y_size: u16,
    #[set]
    pub dest_map_name: String,
    #[set]
    pub to_x: u16,
    #[set]
    pub to_y: u16,
}

impl Warp {
    pub fn new() -> Warp {
        Warp{
            map_name: "".to_string(),
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            dest_map_name: "".to_string(),
            to_x: 0,
            to_y: 0
        }
    }
    pub async fn load_warps() -> HashMap<String, Warp> {
        let semaphore = Semaphore::new(PARALLEL_EXECUTIONS);
        let file = File::open(Path::new(WARP_CONF_PATH)).unwrap();
        let mut reader = BufReader::new(file);
        let mut warps_by_map = Arc::new(Mutex::new(HashMap::<String, Warp>::new()));
        let mut futures : Vec<JoinHandle<()>> = Vec::new();
        for line in reader.lines() {
            if !line.is_ok() {
                break;
            }
            let mut line = line.unwrap();
            if !line.starts_with("npc:") {
                continue;
            }
            line = line.replace("npc: ", "");
            let warp_script_path = line.trim().clone().to_string();

            semaphore.acquire().await.unwrap();
            let res = warps_by_map.clone();
            futures.push(tokio::task::spawn_blocking(move || {
                let warp_script_file_res = File::open(Path::new(&warp_script_path));
                if warp_script_file_res.is_err() {
                    warn!("Not able to load warp script: {}, due to {}", warp_script_path, warp_script_file_res.err().unwrap());
                    return;
                }
                let warp = Warp::parse_warp(&warp_script_file_res.unwrap());
                if warp.is_some() {
                    let warp = warp.unwrap();
                    {
                        let mut res_guard = res.lock().unwrap();
                        res_guard.insert(warp.map_name.clone(), warp.clone());
                    }
                }
            }));
        }
        join_all(futures).await;
        let mut guard = warps_by_map.lock().unwrap();
        let mut res= HashMap::<String, Warp>::new();
        guard.iter().for_each(|(k, v)| {
            res.insert(k.clone(), v.clone());
        });
        res
    }

    fn parse_warp(file: &File) -> Option<Warp> {
        let mut reader = BufReader::new(file);
        let mut warp = Warp::new();
        for line in reader.lines() {
            if !line.is_ok() {
                return None
            }
            let line = line.unwrap();
            if line.starts_with("//") || !line.contains("\twarp\t") {
                continue;
            }
            // A warp "npc" definition is as below
            // anthell01,253,32,0	warp	ant01	2,1,anthell02,34,263
            let line_fragment = line.split("\t").collect::<Vec<&str>>();
            let source_information = line_fragment[0];
            let wapr_and_destination_information = line_fragment[3];
            let source_information_split = source_information.split(",").collect::<Vec<&str>>();
            let wapr_and_destination_information_split = wapr_and_destination_information.split(",").collect::<Vec<&str>>();
            warp.set_map_name(source_information_split[0].to_string());
            warp.set_x(source_information_split[1].parse::<u16>().unwrap());
            warp.set_y(source_information_split[2].parse::<u16>().unwrap());
            warp.set_x_size(wapr_and_destination_information_split[0].parse::<u16>().unwrap());
            warp.set_y_size(wapr_and_destination_information_split[1].parse::<u16>().unwrap());
            warp.set_dest_map_name(wapr_and_destination_information_split[2].to_string());
            warp.set_to_x(wapr_and_destination_information_split[3].parse::<u16>().unwrap());
            warp.set_to_y(wapr_and_destination_information_split[4].parse::<u16>().unwrap());
        }
        Some(warp)
    }
}