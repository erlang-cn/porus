#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n: usize = read!();
    let q: isize = read!();

    let a = &mut DoublyLinkedList::<_>::new();

    for _ in 0..n {
        let b: StringBuffer = read!();
        let name: String = From::from(b);
        let time: isize = read!();
        deque::push_back(a, (name, time));
    }

    let mut sum: isize = 0;

    while !deque::is_empty(a) {
        let (name, time) = deque::pop_front(a);
        if time <= q {
            sum += time;
            writelnf!("{:s} {sum:d}", &name);
        } else {
            sum += q;
            deque::push_back(a, (name, time - q));
        }
    }
}