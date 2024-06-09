mod examples;
mod mutable_ref;
mod rcbox;

use examples::{
    channels, clojoures, custom_structs, longest_common_prevfix, many_threads, share_state,
    smart_pointers, string_ops, sync_async_count,
};

fn clear_screen(enable: bool) {
    if enable {
        print!("{esc}c", esc = 27 as char);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn main() {
    let clear = false;
    println!("#start");

    println!("[longest_common_prevfix]");
    let result = longest_common_prevfix::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
        "floor".to_string(),
    ]);
    println!("{}", result);
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

    rcbox::run();

    mutable_ref::run();

    println!("#end");
}
