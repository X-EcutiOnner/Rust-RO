use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::convert::TryInto;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::{Decompress, FlushDecompress};
use flate2::read::ZlibDecoder;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use log::warn;
use accessor::Setters;
use crate::server::core::Server;
use crate::server::scripts::warps::Warp;
use std::sync::Arc;

static MAP_EXT: &str = ".mcache";
static MAP_DIR: &str = "./maps/pre-re";

static WARP_MASK: u16 = 0b00000100_00000000;

struct Header {
    pub version: i16,
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}

#[derive(Debug)]
pub struct WarpDestination {
    pub map_name: String,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct Map {
    pub x_size: u16,
    pub y_size: u16,
    pub length: i32,
    pub name: String,
    // index in this array will give x and y position of the cell.
    // 2 bytes representing cell type:
    // bit 0 -> walkable
    // bit 1 -> shootable
    // bit 2 -> water
    // bit 3 -> npc
    // bit 4 -> basilica
    // bit 5 -> landprotector
    // bit 6 -> novending
    // bit 7 -> nochat
    // bit 8 -> icewall
    // bit 9 -> noicewall
    // bit 10 -> noskill
    // bit 11 -> warp
    pub cells: Option<Vec<u16>>,
    pub warps: HashMap<usize, Arc<WarpDestination>>,
    pub is_initialized: bool, // maps initialization is lazy, this bool indicate if maps has been initialized or not
}

#[derive(Setters)]
pub struct MapPropertyFlags {
    // PARTY - Show attack cursor on non-party members (PvP)
    #[set]
    pub is_party: bool,
    // GUILD - Show attack cursor on non-guild members (GvG)
    #[set]
    pub is_guild: bool,
    // SIEGE - Show emblem over characters heads when in GvG (WoE castle)
    #[set]
    pub is_siege: bool,
    // USE_SIMPLE_EFFECT - Automatically enable /mineffect
    #[set]
    pub use_simple_effect: bool,
    // DISABLE_LOCKON - Only allow attacks on other players with shift key or /ns active
    #[set]
    pub is_no_lockon: bool,
    // COUNT_PK - Show the PvP counter
    #[set]
    pub count_pk: bool,
    // NO_PARTY_FORMATION - Prevents party creation/modification (Might be used for instance dungeons)
    #[set]
    pub party_lock: bool,
    // BATTLEFIELD - Unknown (Does something for battlegrounds areas)
    #[set]
    pub is_battleground: bool,
    // DISABLE_COSTUMEITEM - Disable costume sprites
    #[set]
    pub is_no_costum: bool,
    // USECART - Allow opening cart inventory (Well force it to always allow it)
    #[set]
    pub is_use_cart: bool,
    // SUNMOONSTAR_MIRACLE - Blocks Star Gladiator's Miracle from activating
    #[set]
    pub is_summonstar_miracle: bool,
    // Unused bits. 1 - 10 is 0x1 length and 11 is 0x15 length. May be used for future settings.
    pub unused: bool,
}

impl MapPropertyFlags {
    pub fn new() -> MapPropertyFlags {
        MapPropertyFlags {
            is_party: false,
            is_guild: false,
            is_siege: false,
            use_simple_effect: false,
            is_no_lockon: false,
            count_pk: false,
            party_lock: false,
            is_battleground: false,
            is_no_costum: false,
            is_use_cart: false,
            is_summonstar_miracle: false,
            unused: false
        }
    }

    pub fn raw(&self) -> u32 {
        (((self.is_party as u32) << 0)
            | ((self.is_guild as u32) << 1)
            | ((self.is_siege as u32) << 2)
            | ((self.use_simple_effect as u32) << 3)
            | ((self.is_no_lockon as u32) << 4)
            | ((self.count_pk as u32) << 5)
            | ((self.party_lock as u32) << 6)
            | ((self.is_battleground as u32) << 7)
            | ((self.is_no_costum as u32) << 8)
            | ((self.is_use_cart as u32) << 9)
            | ((self.is_summonstar_miracle as u32) << 10)
            | ((self.unused as u32) << 11))
    }
}

impl Map {
    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        (x as u32 + y as u32 * self.x_size as u32) as usize
    }
    #[inline]
    pub fn get_pos_of(&self, index: u32) -> (u16, u16) {
        let y: u16 = (index / self.x_size as u32) as u16;
        let x: u16 = (index - (y as u32 * self.x_size as u32) as u32) as u16;
        (x, y)
    }

    pub fn is_cell_walkable(&self, x: u16, y: u16) -> bool {
        if self.cells.is_none() {
            warn!("Cannot call is_cell_walkable as cells are not initialized, returning false");
            return false
        }
        (self.cells.as_ref().unwrap().get(self.get_cell_index_of(x, y)).unwrap() & 0b0000_0000_0000_0001) == 0b0000_0000_0000_0001
    }

    pub fn is_warp_cell(&self, x: u16, y: u16) -> bool {
        if self.cells.is_none() {
            warn!("Cannot call is_warp_cell as cells are not initialized, returning false");
            return false
        }
        let i = self.get_cell_index_of(x, y);
        match self.cells.as_ref().unwrap().get(i) {
            Some(value) => (value & WARP_MASK) == WARP_MASK,
            None => false
        }
    }

    pub fn player_join_map(&mut self, warps: Option<&Vec<Warp>>) {
        if !self.is_initialized {
            self.initialize(warps);
        }
        // TODO maintain a list of player in the map
    }

    fn initialize(&mut self, warps: Option<&Vec<Warp>>) {
        self.set_cells();
        if warps.is_some() {
            self.set_warps(warps.unwrap());
        }
        self.is_initialized = true;
    }

    pub fn set_cells(&mut self) {
        let file_path = Path::join(Path::new(MAP_DIR), format!("{}{}", self.name, MAP_EXT));
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut map_cache_zip_content_buf = Vec::new();
        let mut map_cache_content_buf = Vec::new();
        reader.read_to_end(&mut map_cache_zip_content_buf);
        let mut decoder = ZlibDecoder::new(&map_cache_zip_content_buf[26..]); // skip header
        decoder.read_to_end(&mut map_cache_content_buf).unwrap();

        let mut cells: Vec<u16> = Vec::new();
        for cell in map_cache_content_buf {
            cells.push(match cell {
                0 | 2 | 4 | 6 => 3, // 3 => bytes 0 and byte 1 are set. walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 7, // 7 => bytes 0, 1 ,2 are set. walkable water
                5 => 2, // 2 => byte 1 is set gap, (shootable)
                _ => 0
            })
        }
        self.cells = Some(cells);
    }

    fn set_warps(&mut self, warps: &Vec<Warp>) {
        for warp in warps {
            let start_x = warp.x - warp.x_size;
            let to_x = warp.x + warp.x_size;
            let start_y = warp.y - warp.y_size;
            let to_y = warp.y + warp.y_size;
            let destination = Arc::new(WarpDestination {
                map_name: warp.dest_map_name.clone(),
                x: warp.to_x,
                y: warp.to_y
            });
            for x in start_x..to_x {
                for y in start_y..to_y {
                    let index = self.get_cell_index_of(x, y);
                    let cell = self.cells.as_mut().unwrap().get_mut(index).unwrap();
                    *cell |= WARP_MASK;
                    self.warps.insert(index, destination.clone());
                }
            }
        }
    }

    pub fn load_maps() -> HashMap<String, Map> {
        let mut maps = HashMap::<String, Map>::new();
        let paths = fs::read_dir(MAP_DIR).unwrap();
        for path in paths {
            let start = Instant::now();
            let path = path.as_ref().unwrap();
            let map_name = path.file_name().to_str().unwrap().replace(MAP_EXT, "");
            let file = File::open(path.path()).unwrap();
            let mut reader = BufReader::new(file);
            let mut buf = [0 as u8; 26];
            reader.read(&mut buf);
            let header = Header {
                version: Cursor::new(buf[0..2].to_vec()).read_i16::<LittleEndian>().unwrap(),
                checksum: buf[2..18].try_into().unwrap(),
                x_size: Cursor::new(buf[18..20].to_vec()).read_i16::<LittleEndian>().unwrap(),
                y_size: Cursor::new(buf[20..22].to_vec()).read_i16::<LittleEndian>().unwrap(),
                length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap()
            };
            // TODO validate checksum
            // TODO validate size + length


            let map = Map {
                x_size: header.x_size as u16,
                y_size: header.y_size as u16,
                length: header.length,
                name: map_name.to_string(),
                cells: None,
                warps: Default::default(),
                is_initialized: false
            };
            maps.insert(map.name.clone(), map);
        }
        maps
    }

    pub fn name_without_ext(map_name: String) -> String {
        map_name.replace(".gat", "")
    }
}