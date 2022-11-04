use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{Scope, sleep};
use std::time::{Duration, Instant};
use packets::packets::{Packet, PacketZcNpcackMapmove};
use crate::server::core::position::Position;
use crate::server::core::event::Event;
use crate::server::core::map::{MAP_EXT, MapItem};
use crate::server::core::notification::{CharNotification, Notification};
use crate::server::enums::map_item::MapItemType;
use crate::server::server::Server;
use crate::util::string::StringUtil;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>) {
        loop {
            let start = Instant::now();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        Event::CharacterChangeMap(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
                            if let Some(map) = server_ref.maps.get(event.new_map_name.as_str()) {
                                character.join_and_set_map(map.get_instance(event.new_instance_id));
                                let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

                                let mut new_current_map: [char; 16] = [0 as char; 16];
                                let map_name = format!("{}{}", event.new_map_name, MAP_EXT);
                                map_name.fill_char_array(new_current_map.as_mut());
                                packet_zc_npcack_mapmove.set_map_name(new_current_map);
                                packet_zc_npcack_mapmove.set_x_pos(character.x() as i16);
                                packet_zc_npcack_mapmove.set_y_pos(character.y() as i16);
                                packet_zc_npcack_mapmove.fill_raw();
                                client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.account_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
                                    .expect("Failed to send notification event with PacketZcNpcackMapmove");
                            }
                        }
                        Event::CharacterRemoveFromMap(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            let map = server_ref.maps.get(character.current_map_name()).unwrap();
                            map.get_instance(character.current_map_instance()).remove_character(character.to_map_item());
                        }
                        Event::CharacterUpdatePosition(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
                            character.update_position(event.x, event.y);
                        }
                        Event::CharacterClearFov(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.clear_map_view();
                        }
                        Event::CharacterInsert(character) => {
                            characters.insert(character.char_id, character);
                            info!("inserted char");
                        }
                        Event::CharacterRemove(char_id) => {
                            characters.remove(&char_id);
                        }
                    }
                }
            }
            for (_, character) in characters.iter_mut() {
                // character.load_units_in_fov(server_ref, client_notification_sender_clone.clone())
            }
            sleep(Duration::from_millis(17));
        }
    }

    // pub fn map_item_x_y(&self, map_item: MapItem) -> Option<Position> {
    //     match map_item.object_type() {
    //         MapItemType::Character => {
    //             let characters = self.characters.borrow();
    //             if let Some(character) = characters.get(&map_item.id()) {
    //                return Some(Position{ x: character.x(), y: character.y(), dir: 3 }); // TODO add dir to character
    //             }
    //             None
    //         }
    //         MapItemType::Mob => {
    //
    //         }
    //         MapItemType::Warp => {
    //
    //         }
    //         MapItemType::Unknown => {
    //
    //         }
    //         MapItemType::Npc => {
    //
    //         }
    //     }
    // }
}