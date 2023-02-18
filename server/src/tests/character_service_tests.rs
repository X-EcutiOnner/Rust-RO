use std::sync::Arc;
use crate::repository::CharacterRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct CharacterServiceTestContext {
    test_context: TestContext,
    character_service: CharacterService,
}

fn before_each(character_repository: Arc<dyn CharacterRepository + Sync>) -> CharacterServiceTestContext {
    before_each_with_latch(character_repository, 0)
}


fn before_each_with_latch(character_repository: Arc<dyn CharacterRepository + Sync>, latch_size: usize) -> CharacterServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    CharacterServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        character_service: CharacterService::new(client_notification_sender, persistence_event_sender, character_repository, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::Relaxed;
    use std::time::Duration;
    use async_trait::async_trait;
    use sqlx::Error;
    use tokio::runtime::Runtime;
    use enums::class::JobName;
    use enums::look::LookType;
    use packets::packets::{PacketZcSpriteChange2, PacketZcLongparChange, PacketZcParChange};
    use crate::{assert_sent_packet_in_current_packetver, assert_sent_persistence_event};
    use crate::tests::common::assert_helper::{has_sent_persistence_event, has_sent_notification, NotificationExpectation, SentPacket};
    use crate::tests::character_service_tests::before_each;
    use crate::tests::common::character_helper::{add_item_in_inventory, add_items_in_inventory, create_character};
    use crate::tests::common::mocked_repository;
    use crate::enums::EnumWithStringValue;
    use crate::enums::EnumWithNumberValue;
    use crate::repository::CharacterRepository;
    use crate::server::model::events::game_event::{CharacterChangeMap, CharacterLook, CharacterZeny};
    use crate::server::model::events::persistence_event::{PersistenceEvent, SavePositionUpdate, StatusUpdate};
    use crate::server::model::map_instance::MapInstanceKey;
    use crate::server::model::movement::Movement;
    use crate::server::model::position::Position;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::request_handler::char::handle_char_enter;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::util::tick::get_tick;

    #[test]
    fn test_max_weight() {
        // Given
        let context = before_each(mocked_repository());
        struct WeightExpectation<'a> {
            job: &'a str,
            str: u16,
            expected_max_weight: u32,
        }
        // Note that client side display weight values / 10.
        let expectations = vec![
            WeightExpectation { job: "Novice", str: 1, expected_max_weight: 20300 },
            WeightExpectation { job: "Archer", str: 1, expected_max_weight: 26300 },
            WeightExpectation { job: "Blacksmith", str: 1, expected_max_weight: 30300 },
            WeightExpectation { job: "Swordsman", str: 1, expected_max_weight: 28300 },
            WeightExpectation { job: "Swordsman", str: 50, expected_max_weight: 43000 },
        ];
        for expectation in expectations.iter() {
            let mut character = create_character();
            character.status.str = expectation.str;
            character.status.job = JobName::from_string(expectation.job).value() as u32;
            // When
            let max_weight = context.character_service.max_weight(&character);
            // Then
            assert_eq!(max_weight, expectation.expected_max_weight, "Expected max weight to be {} but was {} for class {}", expectation.expected_max_weight, max_weight, expectation.job);
        }
    }

    #[test]
    fn test_can_carry_weight() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 1;
        character.status.job = JobName::from_string("Novice").value() as u32;
        let phracon = GlobalConfigService::instance().get_item_by_name("Phracon");

        // When
        add_items_in_inventory(&mut character, "Phracon", 80); // Phracon weight is 200
        let can_carry = context.character_service.can_carry_weight(&character, phracon.weight as u32);
        // Then
        assert!(can_carry);

        // When
        add_items_in_inventory(&mut character, "Phracon", 11);
        let can_carry = context.character_service.can_carry_weight(&character, phracon.weight as u32);
        // Then
        assert!(!can_carry)
    }

    #[test]
    fn test_change_map_should_clear_movement() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        character.movements = vec![Movement::new(100, 100, get_tick())];
        assert!(!character.movements.is_empty());
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert!(character.movements.is_empty());
    }

    #[test]
    fn test_change_map_should_update_position() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        character.x = 99;
        character.y = 99;
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert_eq!(character.x(), 120);
        assert_eq!(character.y(), 120);
    }

    #[test]
    fn test_change_map_should_defer_position_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_sent_persistence_event!(context, PersistenceEvent::SaveCharacterPosition(SavePositionUpdate { char_id: character.char_id, account_id: character.account_id, map_name: "geffen.gat".to_string(), x: 120, y: 120 }));
    }

    #[test]
    fn test_change_map_should_update_map() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 1);
        let mut character = create_character();
        character.x = 99;
        character.y = 99;
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert_eq!(character.map_instance_key.map_name(), &"geffen.gat".to_string());
        assert_eq!(character.map_instance_key.map_instance(), 1);
    }

    #[test]
    fn test_change_look_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let character_look = CharacterLook { char_id: character.char_id, look_type: LookType::Hair, look_value: 10 };
        // When
        context.character_service.change_look(character_look, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_eq!(10, character.get_look(LookType::Hair));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "hair".to_string(), value: 10, }));
    }

    #[test]
    fn test_change_look_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let character_look = CharacterLook { char_id: character.char_id, look_type: LookType::Hair, look_value: 10 };
        // When
        context.character_service.change_look(character_look, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id())]));
    }

    #[test]
    fn test_change_sprite_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_sprite(&character, LookType::Hair, 10, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_micros(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id())]));
    }

    #[test]
    fn test_update_zeny_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_zeny = CharacterZeny { char_id: character.char_id, zeny: Some(100) };
        // When
        context.character_service.update_zeny(&runtime, character_zeny, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_eq!(character.status.zeny, 100);
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "zeny".to_string(), value: 100, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcLongparChange::packet_id())]));
    }

    #[test]
    fn test_update_zeny_should_fetch_zeny_when_zeny_amount_is_not_specified() {
        // Given
        struct MockedCharacterRepository {called_fetch_zeny: AtomicBool};
        #[async_trait]
        impl CharacterRepository for MockedCharacterRepository {
            async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
                self.called_fetch_zeny.store(true, Relaxed);
                Ok((50))
            }
        }
        let mocked_character_repository = Arc::new(MockedCharacterRepository { called_fetch_zeny: AtomicBool::new(false) });
        let context = before_each(mocked_character_repository.clone());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_zeny = CharacterZeny { char_id: character.char_id, zeny: None };
        // When
        context.character_service.update_zeny(&runtime, character_zeny, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_micros(200));
        assert_eq!(character.status.zeny, 50);
        assert!(mocked_character_repository.called_fetch_zeny.load(Relaxed));
    }

    #[test]
    fn test_update_base_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(character.status.base_level, 78);
        // When
        context.character_service.update_base_level(&mut character, Some(788), None);
        // Then
        assert_eq!(character.status.base_level, 99);
        // When
        context.character_service.update_base_level(&mut character, None, Some(-6));
        // Then
        assert_eq!(character.status.base_level, 93);
        // When
        context.character_service.update_base_level(&mut character, None, Some(10));
        // Then
        assert_eq!(character.status.base_level, 99);
        // When
        context.character_service.update_base_level(&mut character, None, Some(-150));
        // Then
        assert_eq!(character.status.base_level, 1);
        // When
        context.character_service.update_base_level(&mut character, Some(66), Some(10));
        // Then
        assert_eq!(character.status.base_level, 66);
    }

    #[test]
    fn test_update_base_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(character.status.base_level, 78);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "base_level".to_string(), value: 78, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcParChange::packet_id())]));
    }

    #[test]
    fn test_update_base_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        let delta = context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(delta, 77);
    }

    #[test]
    fn test_update_job_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_job_level(&mut character, Some(68), None);
        // Then
        assert_eq!(character.status.job_level, 68);
        // When
        context.character_service.update_job_level(&mut character, Some(788), None);
        // Then
        assert_eq!(character.status.job_level, 70);
        // When
        context.character_service.update_job_level(&mut character, None, Some(-6));
        // Then
        assert_eq!(character.status.job_level, 64);
        // When
        context.character_service.update_job_level(&mut character, None, Some(5));
        // Then
        assert_eq!(character.status.job_level, 69);
        // When
        context.character_service.update_job_level(&mut character, None, Some(-150));
        // Then
        assert_eq!(character.status.job_level, 1);
        // When
        context.character_service.update_job_level(&mut character, Some(66), Some(10));
        // Then
        assert_eq!(character.status.job_level, 66);
    }

    #[test]
    fn test_update_job_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_job_level(&mut character, Some(68), None);
        // Then
        assert_eq!(character.status.job_level, 68);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_micros(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "job_level".to_string(), value: 68, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcParChange::packet_id())]));
    }

    #[test]
    fn test_update_job_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        let delta = context.character_service.update_job_level(&mut character, Some(68), None);
        // Then
        assert_eq!(delta, 67);
    }

    #[test]
    fn test_change_job_should_update_in_memory() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(character.status.job, JobName::Assassin.value() as u32);
    }

    #[test]
    fn test_change_job_should_defer_db_update() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin);
        // Then
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "class".to_string(), value: JobName::Assassin.value() as u32, }));
    }

    #[test]
    fn test_change_job_should_notify_sprite_change() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id())]));
    }

    #[test]
    fn test_load_units_in_fov_should_add_new_item_in_character_map_view() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then
    }

    #[test]
    fn test_load_units_in_fov_should_remove_out_of_fov_item_from_character_map_view() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then
    }



    #[test]
    fn test_get_status_point_count_for_level() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {level: u16, job: &'a str, expected: u32 };
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", expected: 48,},
            Scenarii { level: 63, job: "Thief", expected: 600,},
            Scenarii { level: 99, job: "Assassin", expected: 1273,},
            Scenarii { level: 1, job: "Novice High", expected: 100,},
            Scenarii { level: 63, job: "Archer High", expected: 652,},
            Scenarii { level: 99, job: "Clown", expected: 1325,},
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.level as u32;
            // When
            let status_point_count = context.character_service.get_status_point_count_for_level(&character);
            // Then
            assert_eq!(status_point_count, scenarii.expected, "Expected character of class {} at level {} to have {} status point but got {}", scenarii.job, scenarii.level, scenarii.expected, status_point_count);
        }
    }

    #[test]
    fn test_get_status_point_allocated() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {level: u16, job: &'a str, str: u16, agi: u16, dex: u16, int: u16, luk: u16, vit: u16 , expected: u32 };
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", str: 19, agi: 3, dex: 0, int: 0, luk: 0, vit: 0, expected: 48,},
            Scenarii { level: 63, job: "Thief", str: 31, agi: 77, dex: 33, int: 0, luk: 0, vit: 0, expected: 594,},
            Scenarii { level: 99, job: "Assassin", str: 85, agi: 99, dex: 44, int: 0, luk: 0, vit: 0, expected: 1266,},
            Scenarii { level: 1, job: "Novice High", str: 33, agi: 0, dex: 0, int: 0, luk: 0, vit: 0, expected: 100,},
            Scenarii { level: 63, job: "Archer High", str: 0, agi: 48, dex: 33, int: 0, luk: 0, vit: 54, expected: 503,},
            Scenarii { level: 99, job: "Clown", str: 0, agi: 0, dex: 99, int: 75, luk: 66, vit: 0, expected: 1324,},
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.level as u32;
            character.status.str = scenarii.str;
            character.status.agi = scenarii.agi;
            character.status.dex = scenarii.dex;
            character.status.int = scenarii.int;
            character.status.luk = scenarii.luk;
            character.status.vit = scenarii.vit;
            // When
            let status_point_count = context.character_service.get_spent_status_point(&character);
            // Then
            assert_eq!(status_point_count, scenarii.expected, "Expected character of class {} at level {} to have {} status point but got {}", scenarii.job, scenarii.level, scenarii.expected, status_point_count);
        }
    }
}