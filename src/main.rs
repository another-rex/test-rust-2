#![allow(unconditional_panic)]
// use std::collections::HashSet;

// use once_cell::sync::Lazy;
use regex::Regex;

// static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new("::h[a-f0-9]{16}$").unwrap());
// static RE_GENERICS: Lazy<Regex> = Lazy::new(|| Regex::new(r"<\w+>").unwrap());

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut acc = 0;
    for i in 0..1 {
        acc += potentially_used_func();
    }

    another_func();
    // let output = unused_func();
    let collector = crossbeam_epoch::Collector::new();
    // let handle = collector.register();
    // let mut guard = handle.pin();
    // guard.flush();
    // guard.repin();
    println!("{}", acc);
}

fn another_func() {
    let regex = regex::Regex::new("asdasdad");
    let b = 3 + potentially_used_func();
    panic!("sadsad")
}

#[inline(always)]
fn potentially_used_func() -> i32 {
    // memoffset::offset_of!()
    let a = 1 + 1;
    let backtrace = std::backtrace::Backtrace::capture().to_string();
    println!("{}", backtrace);
    a
}

// fn dsfsfsdf() {
//     let res = std::fs::read_to_string("test1231322").unwrap();
//     let function_lines = res
//         .lines()
//         .map(|line| line.split(" ").collect::<Vec<&str>>())
//         .filter(|a| a[1].to_lowercase() == "t")
//         .map(|a| a[2])
//         .map(simple_function_names)
//         .collect::<HashSet<_>>();

//     dbg!(function_lines);
// }

// fn simple_function_names(name: &str) -> String {
//     let demanged = rustc_demangle::demangle(name).to_string();
//     let stripped = RE_ID.replace_all(&demanged, "");

//     let removed_generics = RE_GENERICS.replace_all(&stripped, "");
//     // dbg!("asdasd");
//     removed_generics.to_string()
// }

// // fn main() {
// //     println!("sadsa");
// //     let a = C { somedata: 4 };
// //     let res = reqwest::blocking::get("https://osv.dev").unwrap();
// //     // let res = request::("https://osv.dev", &mut std::collections::HashMap::new()).unwrap();
// //     other_item_123(&a);
// // }

// // fn other_item_123(item: &dyn TestTrait) {
// //     item.test();
// //     let mut v = smallvec::SmallVec::<[usize; 16]>::new();
// //     // v.push(self.somedata);
// //     // v.insert_many(0, 5..=item);
// //     dbg!(v);
// //     // item.run_stuff(item);
// // }

// // fn this_does_call() {
// //     let mut v = smallvec::SmallVec::<[usize; 16]>::new();
// //     // v.push(self.somedata);
// //     v.insert_many(0, 5..=10);
// // }

// // trait TestTrait {
// //     fn test(&self);
// //     fn run_stuff(&self, other: i32);
// // }

// // struct C {
// //     somedata: u64,
// // }

// // impl TestTrait for C {
// //     fn test(&self) {
// //         let mut v = smallvec::SmallVec::<[u64; 16]>::new();
// //         v.push(self.somedata);
// //         v.insert_many(0, 5..=7);
// //         dbg!(v);
// //     }

// //     fn run_stuff(&self, other: i32) {
// //         dbg!(other);
// //     }
// // }
