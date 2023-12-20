mod examples;
use examples::{channels, clojoures, many_threads, share_state, sync_async_count};

fn main() {
    println!("#start");

    // Demo 1
    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);

    // Demo 2
    channels::test_channels();

    // Demo 3
    many_threads::create_threads();
    many_threads::create_threads_arc();

    // Demo 4
    share_state::mutex();
    share_state::arc_mutex();

    // Demo 5
    clojoures::clojoures();

    // Demo 6
    examples::string_ops::string_ops();

    println!("#end");
}
