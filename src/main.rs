mod examples;
use examples::{
    channels, clojoures, custom_structs, many_threads, share_state, smart_pointers, string_ops,
    sync_async_count,
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

    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);
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

    println!("#end");
}
