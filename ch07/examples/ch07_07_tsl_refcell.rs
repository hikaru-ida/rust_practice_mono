use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
    static RABBITS: RefCell<HashSet<&'static str>> = {
        let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
        RefCell::new(rb)
    }
);

fn main() {
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    std::thread::spawn(|| RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")))
        .join()
        .expect("Thread error");

    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト"));
    })
}
