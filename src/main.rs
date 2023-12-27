mod examples;
use examples::{channels, clojoures, many_threads, share_state, string_ops, sync_async_count};

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);

    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn main() {
    println!("#start");

    // Demo 1
    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);
    clear_screen();

    // Demo 2
    channels::test_channels();
    clear_screen();

    // Demo 3
    many_threads::create_threads();
    many_threads::create_threads_arc();
    clear_screen();

    // Demo 4
    share_state::mutex();
    share_state::arc_mutex();
    clear_screen();

    // Demo 5
    clojoures::clojoures();
    clear_screen();

    // Demo 6
    string_ops::string_ops();
    string_ops::first_word();
    clear_screen();

    println!("#end");
}
