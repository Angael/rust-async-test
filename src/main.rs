mod examples;
mod mutable_ref;
mod rcbox;

use examples::{
    channels, clojoures, custom_structs, longest_common_prevfix, many_threads, share_state,
    smart_pointers, string_ops, sync_async_count,
};
use std::{thread, time::Duration};

fn clear_screen(enable: bool) {
    if enable {
        print!("{esc}c", esc = 27 as char);
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let clear = false;
    println!("#start");

    run_longest_common_prefix();
    clear_screen(clear);

    run_sync_async_count();
    clear_screen(clear);

    channels::test_channels();
    clear_screen(clear);

    many_threads::create_threads();
    many_threads::create_threads_arc();
    clear_screen(clear);

    share_state::mutex();
    share_state::arc_mutex();
    clear_screen(clear);

    clojoures::clojoures();
    clear_screen(clear);

    string_ops::string_ops();
    string_ops::first_word();
    clear_screen(clear);

    smart_pointers::run();
    clear_screen(clear);

    custom_structs::run();
    clear_screen(clear);

    rcbox::run();
    clear_screen(clear);

    mutable_ref::run();

    println!("#end");
}

fn run_longest_common_prefix() {
    println!("[longest_common_prefix]");

    let strings = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
        "floor".to_string(),
    ];

    // Method 1: using Vec<String>
    let result = longest_common_prevfix::longest_common_prefix(strings.clone());
    println!("Method 1: {}", result);

    // Method 2: using slice reference
    let result2 = longest_common_prevfix::longest_common_prefix2(&strings);
    println!("Method 2: {}", result2);
}

fn run_sync_async_count() {
    println!("[sync_async_count]");
    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);
}
