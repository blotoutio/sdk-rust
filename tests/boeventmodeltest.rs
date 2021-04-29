use blotout::model::event::Event;
use blotout::model::event::Meta;
use blotout::model::event::Payload;
use blotout::model::event::PersonalData;
use blotout::model::manifest::ManifestRoot;
use blotout::model::manifest::ManifestVariable;
use blotout::utility::system_info_manager::SystemInfoManager;
#[test]
fn test_event_model() {
    let events_arr: Vec<Event> = Vec::new();
    let event_model = Payload {
        meta: Meta {
            ..Default::default()
        },
        events: events_arr,
    };
    assert!(Some(event_model).is_some());
}

#[test]
fn test_secure_data() {
    let secure_data = PersonalData {
        ..Default::default()
    };
    assert!(Some(secure_data).is_some());
}

#[test]
fn test_event() {
    let event = Event {
        ..Default::default()
    };
    assert!(Some(event).is_some());
}

#[test]
fn test_manifest_root() {
    let manifest_root = ManifestRoot {
        ..Default::default()
    };
    assert!(Some(manifest_root).is_some());
}

#[test]
fn test_manifest_variable() {
    let manifest_variable = ManifestVariable {
        ..Default::default()
    };
    assert!(Some(manifest_variable).is_some());
}

#[test]
fn test_system_info_manager() {
    let system_info_manager = SystemInfoManager {
        ..Default::default()
    };
    assert!(Some(system_info_manager).is_some());
}
