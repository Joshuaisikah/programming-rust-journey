// ============================================================
// heap.rs — Box<T>, Rc<T>, Arc<T>
// ============================================================
//
// KEY CONCEPTS:
//
//   Box<T>
//     Allocates T on the heap. Owns the data exclusively.
//     Freed when the Box goes out of scope.
//     Use when: recursive types, large values, trait objects (dyn Trait).
//
//   Rc<T>  (Reference Counted — single-threaded)
//     Multiple owners via reference counting.
//     Cloning an Rc bumps a counter; dropping decrements it.
//     Freed when the count reaches zero.
//     Use when: shared ownership in a single thread.
//     NOT Send/Sync — cannot cross thread boundaries.
//
//   Arc<T>  (Atomic Reference Counted — multi-threaded)
//     Same as Rc but uses atomic ops — safe across threads.
//     Slightly more expensive than Rc.
//     Use with Mutex<T> or RwLock<T> for interior mutability.

use crate::display::section;
use std::rc::Rc;
use std::sync::Arc;

/// Demonstrates Box, Rc, and Arc.
pub fn demo_heap() {
    section("BOX / Rc / Arc");


    // TODO: demo 1 — Box<T> for heap allocation
    //   let boxed: Box<i32> = Box::new(42);
    //   println!("boxed value: {}", *boxed);  // auto-deref also works: boxed
    //   // Box is dropped (heap freed) at end of scope
    let bosed: Box<i32> = Box::new(42);
    println!("boxed value: {}", *bosed);
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    if let List::Cons(head, tail) = &list {
        println!("head={head}, tail={tail:?}");
    }
    println!("{:?}", list);

    // TODO: demo 2 — Box for recursive types
    //   #[derive(Debug)]
    //   enum List { Cons(i32, Box<List>), Nil }
    //   let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    //   println!("{:?}", list);
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("count: {}", Rc::strong_count(&a));
    println!("{} {}", a, b);
    use std::thread;
    let shared = Arc::new(vec![1, 2, 3]);
    let clone = Arc::clone(&shared);
    let handle = thread::spawn(move || println!("thread sees: {:?}", clone));
    handle.join().unwrap();
    println!("main sees:   {:?}", shared);

    // TODO: demo 3 — Rc<T> for shared ownership
    //   let a = Rc::new(String::from("shared"));
    //   let b = Rc::clone(&a);   // bumps ref count, does NOT clone the String
    //   println!("count: {}", Rc::strong_count(&a));  // 2
    //   println!("{} {}", a, b);

    // TODO: demo 4 — Arc<T> for shared ownership across threads
    //   use std::thread;
    //   let shared = Arc::new(vec![1, 2, 3]);
    //   let clone  = Arc::clone(&shared);
    //   let handle = thread::spawn(move || println!("thread sees: {:?}", clone));
    //   handle.join().unwrap();
    //   println!("main sees:   {:?}", shared);

    println!("(implement the heap demos above)");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn test_box_stores_value() {
        let b = Box::new(99_i32);
        assert_eq!(*b, 99);
    }

    #[test]
    fn test_box_deref() {
        let b: Box<str> = "hello".into();
        assert_eq!(&*b, "hello");
    }

    #[test]
    fn test_rc_shared_ownership() {
        let a = Rc::new(42_i32);
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(*a, 42);
    }

    #[test]
    fn test_arc_shared_across_threads() {
        use std::thread;
        let data = Arc::new(vec![1, 2, 3]);
        let clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            assert_eq!(*clone, vec![1, 2, 3]);
        });
        handle.join().unwrap();
        assert_eq!(Arc::strong_count(&data), 1);
    }

    #[test]
    fn test_arc_ref_count() {
        let a = Arc::new("shared");
        let b = Arc::clone(&a);
        let c = Arc::clone(&a);
        assert_eq!(Arc::strong_count(&a), 3);
        drop(c);
        assert_eq!(Arc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Arc::strong_count(&a), 1);
    }

    #[test]
    fn test_box_recursive_list() {
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        let List::Cons(head, tail) = &list else { panic!("expected Cons") };
        assert_eq!(*head, 1);
        let List::Cons(next, _) = tail.as_ref() else { panic!("expected second Cons") };
        assert_eq!(*next, 2);
    }
}
