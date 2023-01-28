#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use packets::packets::{PacketZcNotifyAct3, PacketZcNotifyMove, PacketZcNotifyVanish};

use crate::server::core::map::{Map, MAP_EXT, WARP_MASK};
use crate::server::state::mob::{Mob, MobMovement};
use crate::server::core::path::manhattan_distance;
use crate::server::state::status::Status;
use crate::server::core::map_item::{MapItem, MapItemSnapshot};
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::util::coordinate;
use crate::util::packet::chain_packets_raws;
use packets::packets::Packet;


use std::sync::mpsc::SyncSender;
use std::time::{SystemTime, UNIX_EPOCH};
use rathena_script_lang_interpreter::lang::vm::Vm;
use enums::action::ActionType;
use enums::vanish::VanishType;
use crate::enums::EnumWithNumberValue;
use crate::MyUnsafeCell;
use crate::server::events::map_event::MapEvent;
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::map_item::ToMapItem;
use crate::server::{MOB_FOV, Server};
use crate::server::npc::script::Script;
use crate::server::script::ScriptHandler;
use crate::util::cell::MyRef;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

pub struct MapInstanceKey {
    instance_id: u8,
    map_name: [char; 16],
    map_name_string: String,
}

impl MapInstanceKey {
    pub fn map_name(&self) -> &String {
        &self.map_name_string
    }

    pub fn map_name_char(&self) -> [char; 16] {
        self.map_name
    }
    pub fn map_instance(&self) -> u8 {
        self.instance_id
    }

    pub fn new(map_name: String, id: u8) -> Self {
        let mut new_current_map: [char; 16] = [0 as char; 16];
        let map_name = if !map_name.ends_with(MAP_EXT) {
            format!("{}{}", map_name, MAP_EXT)
        } else {
            map_name
        };
        map_name.fill_char_array(new_current_map.as_mut());
        Self {
            map_name: new_current_map,
            map_name_string: map_name,
            instance_id: id
        }
    }
}

pub struct MapInstance {
    pub name: String,
    pub name_with_ext: String,
    pub id: u8,
    pub x_size: u16,
    pub y_size: u16,
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
    // bit 12 -> mob
    pub cells: Vec<u16>,
    pub warps: Arc<Vec<Arc<Warp>>>,
    pub mob_spawns: Arc<Vec<Arc<MobSpawn>>>,
    pub mob_spawns_tracks: RefCell<Vec<MobSpawnTrack>>,
    pub mobs: MyUnsafeCell<HashMap<u32, Mob>>,
    pub scripts: Vec<Arc<Script>>,
    pub map_items: MyUnsafeCell<HashMap<u32, MapItem>>,
    pub client_notification_channel: SyncSender<Notification>,
    pub map_event_notification_sender: SyncSender<MapEvent>,
}

unsafe impl Sync for MapInstance {}
unsafe impl Send for MapInstance {}

pub struct MobSpawnTrack {
    pub spawn_id: u32,
    pub spawned_amount: i16,
    pub mob_respawn_at: Vec<u128>
}

impl MobSpawnTrack {
    pub fn default(spawn_id: u32) -> MobSpawnTrack {
        MobSpawnTrack {
            spawn_id,
            spawned_amount: 0,
            mob_respawn_at: Default::default()
        }
    }

    pub fn increment_spawn(&mut self) {
        self.spawned_amount += 1;
    }
}

impl MapInstance {
    pub fn from_map(map: &Map, server: &Server, id: u8, cells: Vec<u16>, mut map_items: HashMap<u32, MapItem>,
                    map_event_notification_sender: SyncSender<MapEvent>, client_notification_channel: SyncSender<Notification>) -> MapInstance {
        let _cells_len = cells.len();
        let mut scripts = vec![];
        map.scripts.iter().for_each(|script| {
            let (_, instance_reference) = Vm::create_instance(server.vm.clone(), script.class_name.clone(), Box::new(&ScriptHandler), script.constructor_args.clone()).unwrap();
            let mut script = script.clone();
            script.set_instance_reference(instance_reference);
            let script_arc = Arc::new(script);
            map_items.insert(script_arc.id(), script_arc.to_map_item());
            scripts.push(script_arc);
        });
        MapInstance {
            name: map.name.clone(),
            name_with_ext: Map::name_with_ext(map.name.as_str()),
            id,
            x_size: map.x_size,
            y_size: map.y_size,
            cells,
            warps: map.warps.clone(),
            mob_spawns: map.mob_spawns.clone(),
            mob_spawns_tracks: RefCell::new(map.mob_spawns.iter().map(|spawn| MobSpawnTrack::default(spawn.id)).collect::<Vec<MobSpawnTrack>>()),
            mobs: Default::default(),
            map_items: MyUnsafeCell::new(map_items),
            scripts,
            map_event_notification_sender,
            client_notification_channel
        }
    }

    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        coordinate::get_cell_index_of(x, y, self.x_size)
    }

    pub fn id(&self) -> u8{
        self.id
    }


    pub fn is_cell_walkable(&self, x: u16, y: u16) -> bool {
        if self.cells.is_empty() {
            warn!("Cannot call is_cell_walkable as cells are not initialized, returning false");
            return false
        }
        (self.cells.get(self.get_cell_index_of(x, y)).unwrap() & 0b0000_0000_0000_0001) == 0b0000_0000_0000_0001
    }

    pub fn is_warp_cell(&self, x: u16, y: u16) -> bool {
        if self.cells.is_empty() {
            warn!("Cannot call is_warp_cell as cells are not initialized, returning false");
            return false
        }
        let i = self.get_cell_index_of(x, y);
        match self.cells.get(i) {
            Some(value) => (value & WARP_MASK) == WARP_MASK,
            None => false
        }
    }

    pub fn spawn_mobs(&self) {
        for mob_spawn in self.mob_spawns.iter() {
            let mut mob_spawns_mut = self.mob_spawns_tracks.borrow_mut();
            let mob_spawn_track = mob_spawns_mut.iter_mut().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap();
            if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                continue;
            }
            if mob_spawn.has_delay() {
                // TODO check when respawn is planned
            }
            let mut cell: (u16, u16);
            let spawned = mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount;
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {
                    // if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(self.cells.as_ref(), self.x_size);
                }
                let mob_id = Server::generate_id(&mut self.map_items.borrow_mut());
                let mob = Mob::new(mob_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone(),
                                   MapInstanceKey::new(self.name.clone(), self.id),
                                   Status::from_mob_model(&mob_spawn.info));

                // TODO: On mob dead clean up should be down also for items below
                self.insert_item(mob.to_map_item());
                self.mobs.borrow_mut().insert(mob_id, mob);
                // END
                mob_spawn_track.increment_spawn();
            }
        }
    }

    pub fn update_mobs_fov(&self, characters: Vec<MapItemSnapshot>) {
        for (_, mob) in self.mobs.borrow_mut().iter_mut() {
            let mut viewed_chars: Vec<MapItem> = Vec::with_capacity(characters.len());
            for character in characters.iter() {
                if manhattan_distance(character.x(), character.y(), mob.x(), mob.y()) <= MOB_FOV {
                    viewed_chars.push(character.map_item());
                }
            }
            mob.update_map_view(viewed_chars);
        }
    }

    pub fn mobs_action(&self) {
        let start_time = get_tick_client();
        let start_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut mob_movements: Vec<MobMovement> = Vec::with_capacity(self.mobs.borrow().len() / 2);
        for mob in self.mobs.borrow_mut().values_mut() {
            if let Some(mob_movement) = mob.action_move(self, &self.cells, self.x_size, self.y_size, start_at) {
                mob_movements.push(mob_movement);
            }
        }
        for mob_movement in mob_movements {
            let mut packet_zc_notify_move = PacketZcNotifyMove::default();
            packet_zc_notify_move.set_gid(mob_movement.id);
            packet_zc_notify_move.move_data = mob_movement.from.to_move_data(&mob_movement.to);
            packet_zc_notify_move.set_move_start_time(start_time);
            packet_zc_notify_move.fill_raw();
            debug!("Mob moving from {} to {}. Notify area around {},{}", mob_movement.from, mob_movement.to, mob_movement.from.x, mob_movement.from.y);
            self.client_notification_channel.send(Notification::Area(
                    AreaNotification::new(self.name_with_ext.clone(), self.id(),
                                          AreaNotificationRangeType::Fov { x: mob_movement.from.x, y: mob_movement.from.y, exclude_id: None },
                                          packet_zc_notify_move.raw))).expect("Fail to send client notification");
        }
    }

    pub fn mob_die(&self, id: u32) {
        let mut mobs = self.mobs.borrow_mut();
        let mob = mobs.get(&id).unwrap();
        let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
        packet_zc_notify_vanish.set_gid(mob.id);
        packet_zc_notify_vanish.set_atype(VanishType::Die.value() as u8);
        packet_zc_notify_vanish.fill_raw();
        self.client_notification_channel.send(Notification::Area(
            AreaNotification::new(self.name_with_ext.clone(), self.id(),
                                  AreaNotificationRangeType::Fov { x: mob.x, y: mob.y, exclude_id: None },
                                  packet_zc_notify_vanish.raw))).expect("Fail to send client notification");
        self.remove_item(mob.to_map_item());
        mobs.remove(&id);
    }

    pub fn get_warp_at(&self, x: u16, y: u16) -> Option<Arc<Warp>> {
        for warp in self.warps.iter() {
            if x >= warp.x - warp.x_size && x <= warp.x + warp.x_size
                && y >= warp.y - warp.y_size && y <= warp.y + warp.y_size {
                return Some(warp.clone());
            }
        }
        None
    }

    pub fn insert_item(&self, map_item: MapItem) {
        self.map_items.borrow_mut().insert(map_item.id(), map_item);
    }

    pub fn remove_item(&self, map_item: MapItem) {
        self.remove_item_with_id(map_item.id());
    }

    pub fn remove_item_with_id(&self, id: u32) {
        self.map_items.borrow_mut().remove(&id);
    }

    pub fn notify_event(&self, map_event: MapEvent) {
        self.map_event_notification_sender.send(map_event).unwrap()
    }

    pub fn get_mob(&self, mob_id: u32) -> Option<MyRef<Mob>> {
        if self.mobs.borrow().get(&mob_id).is_some() {
            Some(MyRef::map(self.mobs.borrow(), |mobs| mobs.get(&mob_id).unwrap()))
        } else {
            None
        }
    }

    pub fn get_warp(&self, warp_id: u32) -> Option<Arc<Warp>> {
        for warp in self.warps.iter() {
            if warp.id == warp_id {
                return Some(warp.clone())
            }
        }
        None
    }

    pub fn get_script(&self, script_id: u32) -> Option<Arc<Script>> {
        for script in self.scripts.iter() {
            if script.id() == script_id {
                return Some(script.clone())
            }
        }
        None
    }

    #[inline]
    pub fn get_fov_start_x(&self, x: u16, range: u16) -> u16 {
        if range > x {
            return 0
        }
        x - range
    }

    #[inline]
    pub fn get_fov_start_y(&self, y: u16, range: u16) -> u16 {
        if range > y {
            return 0
        }
        y - range
    }

    #[inline]
    pub fn get_item_x_from_fov(&self, x: u16, range: u16, i: u16) -> u16 {
        let x = self.get_fov_start_x(x, range) + i;
        if x >= self.x_size {
            return self.x_size - 1;
        }
        x
    }

    #[inline]
    pub fn get_item_y_from_fov(&self, y: u16, range: u16, j: u16) -> u16 {
        let y = self.get_fov_start_y(y, range) + j;
        if y >= self.y_size {
            return self.y_size - 1;
        }
        y
    }
}