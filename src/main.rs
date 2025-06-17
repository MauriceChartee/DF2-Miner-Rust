mod data_loader; // Import the module from loader.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = data_loader::load_ocel("C:\\Users\\Postb\\Documents\\GitHub\\DF2-Miner-Rust\\data\\running-example.jsonocel")?;
    println!("OCEL log loaded.");
    println!("Number of events: {}", log.events.len());
    println!("Attributes: {:?}", log.global_log.attribute_names);
    println!("Object types: {:?}", log.global_log.object_types);
    for (id, event) in log.events.iter().take(5) {
    println!("Event {id}: activity={}, timestamp={}, objects={:?}, attributes={:?}", event.activity, event.timestamp, event.omap, event.vmap);
    }
    Ok(())
}