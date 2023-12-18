mod examples;
use examples::{channels, many_threads, sync_async_count};

fn main() {
    println!("#start");

    // Demo 1
    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);

    // Demo 2
    channels::test_channels();

    // Demo 3
    many_threads::create_threads();

    println!("#end");
}
