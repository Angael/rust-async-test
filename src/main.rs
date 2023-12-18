mod examples;
use examples::{channels, sync_async_count};

fn main() {
    println!("#start");

    // Demo 1
    let vector = sync_async_count::count();
    println!("vector: {:?}", vector);

    // Demo 2
    channels::test_channels();

    println!("#end");
}
