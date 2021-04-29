use blotout::model::boeventmodel::BOEvent;
use blotout::model::boeventmodel::BOEventModel;
use blotout::model::boeventmodel::BOEventSecureDataModel;
use blotout::model::boeventmodel::BOMeta;
use blotout::model::boeventmodel::BOSecureData;
use blotout::model::boeventmodel::BOSessionInfo;
use blotout::model::bomanifestmodel::BOManifestRoot;
use blotout::model::bomanifestmodel::BOManifestVariable;
use blotout::utility::bosysteminfomanager::BOSystemInfoManager;
#[test]
fn test_event_model() {
    let events_arr: Vec<BOEvent> = Vec::new();
    let event_model = BOEventModel {
        meta: BOMeta {
            ..Default::default()
        },
        events: events_arr,
    };
    assert!(Some(event_model).is_some());
}

#[test]
fn test_session_info() {
    let session_info = BOSessionInfo {
        ..Default::default()
    };
    assert!(Some(session_info).is_some());
}

#[test]
fn test_secure_data() {
    let secure_data = BOSecureData {
        ..Default::default()
    };
    assert!(Some(secure_data).is_some());
}

#[test]
fn test_secure_data_model() {
    let secure_data_model = BOEventSecureDataModel {
        ..Default::default()
    };
    assert!(Some(secure_data_model).is_some());
}

#[test]
fn test_event() {
    let event = BOEvent {
        ..Default::default()
    };
    assert!(Some(event).is_some());
}

#[test]
fn test_manifest_root() {
    let manifest_root = BOManifestRoot {
        ..Default::default()
    };
    assert!(Some(manifest_root).is_some());
}

#[test]
fn test_manifest_variable() {
    let manifest_variable = BOManifestVariable {
        ..Default::default()
    };
    assert!(Some(manifest_variable).is_some());
}

#[test]
fn test_system_info_manager() {
    let system_info_manager = BOSystemInfoManager {
        ..Default::default()
    };
    assert!(Some(system_info_manager).is_some());
}
