use chrono::Local;
use rustmatica::Litematic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let path = path.as_deref().unwrap_or("test_files/axolotl.litematic");
    let donut: Litematic = Litematic::read_file(path)?;

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

    if let Some(img) = donut.preview_image() {
        // display preview image using `viuer`
        let config = viuer::Config {
            transparent: true,
            absolute_offset: false,
            width: Some(30),
            ..Default::default()
        };
        // `viuer` is currently using an incompatible outdated version of `image`
        // viuer::print(img, &config);
        // so we need to save the image as a file instead
        img.save("tmp.png")?;
        viuer::print_from_file("tmp.png", &config)?;
        std::fs::remove_file("tmp.png")?;
    }

    Ok(())
}
