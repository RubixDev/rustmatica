use rustmatica::{chrono::Local, Litematic};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let donut = Litematic::read_file("test_files/axolotl.litematic")?;

    println!("Name:                  {}", donut.name);
    println!("Description:           {}", donut.description);
    println!("Author:                {}", donut.author);
    println!("Regions:               {}", donut.regions.len());
    println!(
        "Region Names:          {}",
        donut
            .regions
            .iter()
            .map(|r| r.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "Created at (UTC):      {}",
        donut.time_created.format("%H:%M %d.%m.%Y")
    );
    println!(
        "Created at (Local):    {}",
        donut
            .time_created
            .with_timezone(&Local)
            .format("%H:%M %d.%m.%Y")
    );
    println!(
        "Last modified (UTC):   {}",
        donut.time_modified.format("%H:%M %d.%m.%Y")
    );
    println!(
        "Last modified (Local): {}",
        donut
            .time_modified
            .with_timezone(&Local)
            .format("%H:%M %d.%m.%Y")
    );
    println!(
        "Size:                  {}x{}x{}",
        donut.enclosing_size().x,
        donut.enclosing_size().y,
        donut.enclosing_size().z
    );
    println!("Blocks:                {}", donut.total_blocks());
    println!("Volume:                {}", donut.total_volume());

    Ok(())
}
