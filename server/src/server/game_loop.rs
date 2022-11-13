use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{Scope, sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tokio::runtime::Runtime;

use packets::packets::{Packet, PacketZcNotifyPlayermove, PacketZcNpcackMapmove};

use crate::Map;
use crate::server::core::character::Character;
use crate::server::core::character_movement::Movement;
use crate::server::core::event::{CharacterChangeMap, Event};
use crate::server::core::map::{MAP_EXT, MapItem, ToMapItem};
use crate::server::core::map_event::MapEvent::SpawnMob;
use crate::server::core::notification::{CharNotification, Notification};
use crate::server::core::position::Position;
use crate::server::enums::map_item::MapItemType;
use crate::server::server::Server;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick;

const MOVEMENT_TICK_RATE: u128 = 20;
const GAME_TICK_RATE: u128 = 40;

// jouer avec ça pour voir si ça change quelque chose
impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, runtime: Runtime) {
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        Event::CharacterChangeMap(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
                            if let Some(map_instance) = server_ref.get_map_instance(&event.new_map_name, event.new_instance_id) {
                                character.join_and_set_map(map_instance);
                                let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

                                let mut new_current_map: [char; 16] = [0 as char; 16];
                                let map_name = format!("{}{}", event.new_map_name, MAP_EXT);
                                map_name.fill_char_array(new_current_map.as_mut());
                                packet_zc_npcack_mapmove.set_map_name(new_current_map);
                                let new_position = event.new_position.unwrap();
                                packet_zc_npcack_mapmove.set_x_pos(new_position.x as i16);
                                packet_zc_npcack_mapmove.set_y_pos(new_position.y as i16);
                                packet_zc_npcack_mapmove.fill_raw();
                                client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.account_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
                                    .expect("Failed to send notification event with PacketZcNpcackMapmove");

                                server_ref.insert_map_item(character.account_id, character.to_map_item());
                                character.update_position(new_position.x, new_position.y);
                                character.clear_map_view();
                                character.loaded_from_client_side = false;
                                Self::save_char_position(&server_ref, &runtime, character);
                            } else {
                                error!("Can't change map to {} {}", event.new_map_name, event.new_instance_id);
                            }
                        }
                        Event::CharacterRemoveFromMap(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.movements = vec![];
                            // if let Some(map_instance) = server_ref.get_map_instance(character.current_map_name(), character.current_map_instance()) {
                            //     map_instance.remove_character(character.to_map_item());
                            // }
                        }
                        Event::CharacterClearFov(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.clear_map_view();
                        }
                        Event::CharacterRemove(char_id) => {
                            characters.remove(&char_id);
                        }
                        Event::CharacterLoadedFromClientSide(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.loaded_from_client_side = true;
                            character.clear_map_view();
                        }
                        Event::CharacterMove(_) => {
                            // handled by dedicated thread
                        }
                        Event::CharacterClearMove(_) => {
                            // handled by dedicated thread
                        }
                    }
                }
            }
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
                character.load_units_in_fov(server_ref.as_ref(), client_notification_sender_clone.clone())
            }
            for (_, map) in server_ref.maps.iter() {
                for instance in map.instances() {
                    instance.notify_event(SpawnMob);
                }
            }
            sleep(Duration::from_millis((GAME_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(GAME_TICK_RATE)) as u64));
        }
    }

    fn save_char_position(server_ref: &Server, runtime: &Runtime, character: &mut Character) {
        let repository = server_ref.repository.clone();
        let account_id = character.account_id;
        let char_id = character.char_id;
        let mapname = character.current_map_name().clone();
        let x = character.x();
        let y = character.y();
        runtime.spawn(async move {
            repository.character_save_position(account_id, char_id, Map::name_without_ext(&mapname), x, y).await
        });
    }

    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, runtime: Runtime) {
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_movement_task() {
                for task in tasks {
                    match task {
                        Event::CharacterClearMove(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.clear_movement();
                        }
                        Event::CharacterMove(character_movement) => {
                            let character = characters.get_mut(&character_movement.char_id).unwrap();
                            let speed = character.status.speed;
                            let maybe_previous_movement = character.pop_movement();
                            character.movements = character_movement.path;
                            let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
                            if let Some(movement) = character.peek_mut_movement() {
                                if let Some(previous_movement) = maybe_previous_movement {
                                    debug!("change path! was {} will {}, move at {}",previous_movement.position(), movement.position(), previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                    // movement.set_move_at(previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                    movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()) + MOVEMENT_TICK_RATE);
                                } else {
                                    movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()));
                                    debug!("will move at {}", movement.move_at());
                                }
                                packet_zc_notify_playermove.set_move_start_time(movement.move_at() as u32); // todo: time conversion check on client side ???
                            }

                            packet_zc_notify_playermove.set_move_data(character_movement.current_position.to_move_data(&character_movement.destination));
                            packet_zc_notify_playermove.fill_raw();
                            client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.account_id, std::mem::take(packet_zc_notify_playermove.raw_mut()))))
                                .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                        }
                        _ => {
                            // handled by game loop thread
                        }
                    }
                }
            }

// If movement not smooth:
// teleport in front -> server movement faster than client movement
// teleport back -> server movement slower than client movement

            let mut character_finished_to_move = vec![];
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.is_moving()) {
                let speed = character.status.speed;
                if let Some(movement) = character.peek_movement() {
                    if tick >= movement.move_at() {
                        debug!("move {} at {}", movement.position(), movement.move_at());
                        let movement = character.pop_movement().unwrap();
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.get_map_instance_from_character(&character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                server_ref.add_to_next_tick(Event::CharacterChangeMap(CharacterChangeMap {
                                    char_id: character.char_id,
                                    new_map_name: warp.dest_map_name.clone(),
                                    new_instance_id: 0,
                                    new_position: Some(Position { x: warp.to_x, y: warp.to_y, dir: movement.position().dir }),
                                    old_map_name: None,
                                    old_position: None,
                                }));
                                character.clear_movement();
                                continue;
                            }
                        }
                        if let Some(next_movement) = character.peek_mut_movement() {
                            next_movement.set_move_at(tick + Movement::delay(speed, next_movement.is_diagonal()))
                        } else {
                            character_finished_to_move.push(character);
                        }
                    }
                }
            }
            for character in character_finished_to_move {
                Self::save_char_position(&server_ref, &runtime, character);
            }
            sleep(Duration::from_millis((MOVEMENT_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(MOVEMENT_TICK_RATE)) as u64));
        }
    }
}

