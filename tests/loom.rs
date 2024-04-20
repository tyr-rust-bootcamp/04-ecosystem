use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use loom::sync::Arc;
use loom::thread;

#[test]
#[should_panic]
fn buggy_concurrent_inc() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();
                thread::spawn(move || {
                    let curr = num.load(Acquire);
                    // This is a bug! this is not atomic!
                    num.store(curr + 1, Release);

                    // fix
                    // num.fetch_add(1, Relaxed);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}
