extern crate saturn;

use saturn::db::db::DB;



#[test]
fn should_create_new_graphs() {
    let mut db: DB = DB::new();

    let res = db.create(&"test".to_string());
    let succ = match res {
        Ok(_) => true,
        Err(_) => false
    };

    let fail = match db.create(&"test".to_string()) {
        Ok(_) => false,
        Err(_) => true
    };

    assert!(succ == true);
    assert!(fail == true);
    assert!(db.existing.contains_key(&"test".to_string()) == true);
}

#[test]
fn should_show_the_graphs() {
    let mut db: DB = DB::new();
    db.create(&"test".to_string());
    db.create(&"fun-ness".to_string());
    let res = match db.show() {
        Ok(a) => true,
        _ => false
    };

    assert!(res == true);
}
