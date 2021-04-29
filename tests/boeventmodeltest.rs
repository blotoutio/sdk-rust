use blotout::model::boeventmodel::BoEvent;
use blotout::model::boeventmodel::BoEventModel;
use blotout::model::boeventmodel::BoEventSecureDataModel;
use blotout::model::boeventmodel::BoMeta;
use blotout::model::boeventmodel::BoSecureData;
use blotout::model::bomanifestmodel::ManifestRoot;
use blotout::model::bomanifestmodel::ManifestVariable;
use blotout::utility::system_info_manager::SystemInfoManager;
#[test]
fn test_event_model() {
    let events_arr: Vec<BoEvent> = Vec::new();
    let event_model = BoEventModel {
        meta: BoMeta {
            ..Default::default()
        },
        events: events_arr,
    };
    assert!(Some(event_model).is_some());
}

#[test]
fn test_secure_data() {
    let secure_data = BoSecureData {
        ..Default::default()
    };
    assert!(Some(secure_data).is_some());
}

#[test]
fn test_secure_data_model() {
    let secure_data_model = BoEventSecureDataModel {
        ..Default::default()
    };
    assert!(Some(secure_data_model).is_some());
}

#[test]
fn test_event() {
    let event = BoEvent {
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
