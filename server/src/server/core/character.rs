use std::any::Any;
use std::cmp;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use packets::packets::{PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use crate::server::core::map::{MapItem};
use crate::server::core::movement::Position;
use crate::server::core::session::Session;
use crate::server::server::{PLAYER_FOV_SLICE_LEN, PLAYER_FOV};
use packets::packets::Packet;
use crate::util::coordinate;
use crate::util::string::StringUtil;
use std::io::Write;
use accessor::Setters;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::status::Status;

#[derive(Setters)]
pub struct CharacterSession {
    #[set]
    pub name: String,
    pub status: Status,
    #[set]
    pub char_id: u32,
    pub current_map: RwLock<Option<Arc<MapInstance>>>,
    pub current_map_name: RwLock<[char; 16]>,
    pub current_position: RwLock<Position>,
    pub movement_task_id: RwLock<Option<u128>>,
    pub map_view: RwLock<Vec<Option<Arc<dyn MapItem>>>>,
    pub self_ref: RwLock<Option<Arc<CharacterSession>>>,
}

impl MapItem for CharacterSession {
    fn id(&self) -> u32 {
        self.char_id
    }

    fn client_item_class(&self) -> i16 {
        todo!() // TODO return job id
    }

    fn object_type(&self) -> i16 {
        0
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn x(&self) -> u16 {
        let current_position_guard = read_lock!(self.current_position);
        current_position_guard.x
    }

    fn y(&self) -> u16 {
        let current_position_guard = read_lock!(self.current_position);
        current_position_guard.y
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CharacterSession {
    pub fn set_self_ref(&self, self_ref: Arc<CharacterSession>) {
        let mut self_ref_guard = write_lock!(self.self_ref);
        *self_ref_guard = Some(self_ref);
    }
    pub fn update_x_y(&self, x: u16, y: u16) {
        let mut current_position_guard = write_lock!(self.current_position);
        current_position_guard.x = x;
        current_position_guard.y = y;
    }
    pub fn set_current_map_name(&self, new_name: [char; 16]) {
        let mut current_map_name_guard = write_lock!(self.current_map_name);
        *current_map_name_guard = new_name;
    }

    pub fn get_pos_index(&self) -> usize {
        let current_map_guard = read_lock!(self.current_map);
        let current_position_guard = read_lock!(self.current_position);
        coordinate::get_cell_index_of(current_position_guard.y, current_position_guard.y, current_map_guard.as_ref().unwrap().x_size)
    }

    pub fn change_map(&self, map_instance: Arc<MapInstance>) {
        let mut map_view_guard = write_lock!(self.map_view);
        *map_view_guard = vec![None; PLAYER_FOV_SLICE_LEN]; // TODO reset map_view of MapItem present in this map view
        self.remove_from_existing_map();
        self.join_and_set_map(map_instance);
    }

    fn remove_from_existing_map(&self) {
        let current_map_guard = read_lock!(self.current_map);
        if current_map_guard.is_some() {
            let map_instance_ref = current_map_guard.as_ref().unwrap();
            let x_size = map_instance_ref.x_size;
            let current_position_guard = read_lock!(self.current_position);
            map_instance_ref.remove_item_at(coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, x_size));
        }
    }

    fn join_and_set_map(&self, map_instance: Arc<MapInstance>) {
        self.set_current_map(map_instance.clone());
        let pos_index = self.get_pos_index();
        let self_ref_guard = read_lock!(self.self_ref);
        map_instance.insert_item_at(pos_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn update_position(&self, x: u16, y: u16) {
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        {
            let current_position_guard = read_lock!(self.current_position);
            let old_position_index = coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, map_ref.x_size);
            map_ref.remove_item_at(old_position_index);
        }
        self.update_x_y(x, y);
        let current_position_guard = read_lock!(self.current_position);
        let new_position_index = coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, map_ref.x_size);
        let self_ref_guard = read_lock!(self.self_ref);
        map_ref.insert_item_at(new_position_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn get_current_map_name(&self) -> String {
        let current_map_name_guard = read_lock!(self.current_map_name);
        current_map_name_guard.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&self, id: u128) {
        let mut movement_task_id_guard = write_lock!(self.movement_task_id);
        *movement_task_id_guard = Some(id);
    }
    pub fn set_current_map(&self, current_map: Arc<MapInstance>) {
        let mut current_map_guard = write_lock!(self.current_map);
        *current_map_guard = Some(current_map);
    }

    pub fn get_fov_start_x(&self) -> u16 {
        cmp::max(self.x() - PLAYER_FOV, 0)
    }

    pub fn get_fov_start_y(&self) -> u16 {
        cmp::max(self.y() - PLAYER_FOV, 0)
    }

    pub fn get_item_x_from_fov(&self, i: usize) -> u16 {
        self.get_fov_start_x() + i as u16
    }

    pub fn get_item_y_from_fov(&self, j: usize) -> u16 {
        self.get_fov_start_y() + j as u16
    }

    // TODO try to optimize, method below take ~0.5ms to execute (peak at 1.5ms)
    pub fn load_units_in_fov(&self, session_guard: &RwLockReadGuard<Session>) {
        let mut new_map_view = vec![None; PLAYER_FOV_SLICE_LEN];
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        let mut map_view_guard = write_lock!(self.map_view);
        let mut previous_item_ids: Vec<u32> = vec![];
        let mut seen_items_ids: Vec<u32> = vec![];

        for item in map_view_guard.iter() {
            if item.is_some() {
                let item = item.as_ref().unwrap();
                previous_item_ids.push(item.id());
            }
        }

        for i in 0..(PLAYER_FOV * 2) {
            for j in 0..(PLAYER_FOV * 2) {
                let x = self.get_item_x_from_fov(i as usize);
                let y = self.get_item_y_from_fov(j as usize);
                if map_ref.is_warp_cell(x, y) {
                    let warp = map_ref.get_warp_at(x, y).unwrap();
                    seen_items_ids.push(warp.id());
                    if !previous_item_ids.contains(&warp.id()) {
                        let mut warp_name = [0 as char; 24];
                        warp.name.fill_char_array(warp_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(warp.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(6);
                        packet_zc_notify_standentry.set_aid(warp.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x: warp.x, y: warp.y, dir: 0 }.to_pos());
                        packet_zc_notify_standentry.set_name(warp_name);
                        packet_zc_notify_standentry.fill_raw();

                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                    new_map_view[coordinate::get_cell_index_of(i, j, PLAYER_FOV)] = Some(warp as Arc<dyn MapItem>);
                }
                let map_item = map_ref.get_map_item_at(x, y);
                if map_item.is_some() {
                    let map_item = map_item.unwrap();
                    if map_item.object_type() != 5 {
                        continue;
                    }
                    // info!("{{{}:{}}},{{{}:{}}} {},{}", self.get_fov_start_x(), self.get_fov_start_y(), self.get_fov_start_x()  + (PLAYER_FOV * 2), self.get_fov_start_y() + (PLAYER_FOV * 2), self.x(), self.y()  );
                    // info!("See mob at {},{} index {}, id {} (inner {},{} - index {})", map_item.x(), map_item.y(),  coordinate::get_cell_index_of(map_item.x(), map_item.y(), 400), map_item.id(),  x, y, coordinate::get_cell_index_of(x, y, 400));
                    new_map_view[coordinate::get_cell_index_of(i, j, PLAYER_FOV)] = Some(map_item.clone());
                    seen_items_ids.push(map_item.id());
                    if !previous_item_ids.contains(&map_item.id()) {
                        let mut mob_name = [0 as char; 24];
                        map_item.name().fill_char_array(mob_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(map_item.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(map_item.object_type() as u8);
                        packet_zc_notify_standentry.set_clevel(3);
                        packet_zc_notify_standentry.set_aid(map_item.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x, y, dir: 3 }.to_pos());
                        packet_zc_notify_standentry.set_name(mob_name);
                        packet_zc_notify_standentry.fill_raw();
                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                }
            }
        }
        *map_view_guard = new_map_view;
        for id in previous_item_ids {
            if !seen_items_ids.contains(&id) {
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(id);
                packet_zc_notify_vanish.fill_raw();
                let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                socket_send!(tcp_stream, packet_zc_notify_vanish.raw());
            }
        }
    }
}