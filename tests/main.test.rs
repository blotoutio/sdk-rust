use blotout::utility::bofilemanager::BOSHAREDFILEINSTANCE;

#[test]
pub fn check_unique_id() {
    let user_id = BOSHAREDFILEINSTANCE.lock().unwrap().get_user_id();
    assert!(user_id);
}
