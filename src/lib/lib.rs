use regex::Regex;

// static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new("::h[a-f0-9]{16}$").unwrap());
// static RE_GENERICS: Lazy<Regex> = Lazy::new(|| Regex::new(r"<\w+>").unwrap());

// pub fn exported_func() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     let mut acc = 0;
//     for _ in 0..1 {
//         acc += potentially_used_func();
//     }

//     let a = another_func();
//     dbg!(a);
//     // let output = unused_func();
//     let collector = crossbeam_epoch::Collector::new();
//     drop(collector);
//     // let handle = collector.register();
//     // let mut guard = handle.pin();
//     // guard.flush();
//     // guard.repin();
//     println!("{}", acc);
// }

// fn another_func() -> i32 {
//     let regex = regex::Regex::new("asdasdad");
//     dbg!(regex).unwrap();
//     let b: i32 = 3 + potentially_used_func();
//     return b;
// }

// #[inline(always)]
// fn potentially_used_func() -> i32 {
//     // memoffset::offset_of!()
//     let a = 1 + 1;
//     // let backtrace = std::backtrace::Backtrace::capture().to_string();
//     // println!("{}", backtrace);
//     a
// }

pub fn a_cool_func_name() {
    println!("Hey, you called a cool function")
}

fn a_not_called_func() {
    println!("Hey, you shouldn't be here!")
}
