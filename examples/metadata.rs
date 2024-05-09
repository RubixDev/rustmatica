use chrono::Local;
use rustmatica::Litematic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let path = path.as_deref().unwrap_or("test_files/axolotl.litematic");
    let schem: Litematic = Litematic::read_file(path)?;

    println!("Name:                   {}", schem.metadata.name);
    println!("Description:            {}", schem.metadata.description);
    println!("Author:                 {}", schem.metadata.author);
    println!("Regions:                {}", schem.regions.len());
    println!(
        "Region Names:           {}",
        schem
            .regions
            .iter()
            .map(|r| r.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "Created at (UTC):       {}",
        schem.metadata.time_created.format("%H:%M %d.%m.%Y")
    );
    println!(
        "Created at (Local):     {}",
        schem
            .metadata
            .time_created
            .with_timezone(&Local)
            .format("%H:%M %d.%m.%Y")
    );
    println!(
        "Last modified (UTC):    {}",
        schem.metadata.time_modified.format("%H:%M %d.%m.%Y")
    );
    println!(
        "Last modified (Local):  {}",
        schem
            .metadata
            .time_modified
            .with_timezone(&Local)
            .format("%H:%M %d.%m.%Y")
    );
    println!(
        "Size:                   {}x{}x{}",
        schem.enclosing_size().x,
        schem.enclosing_size().y,
        schem.enclosing_size().z
    );
    println!("Blocks:                 {}", schem.total_blocks());
    println!("Volume:                 {}", schem.total_volume());
    println!("Format version:         {}", schem.metadata.version);
    println!("Format sub-version:     {:?}", schem.metadata.sub_version);
    println!(
        "Minecraft data version: {}",
        schem.metadata.minecraft_data_version
    );

    if let Some(img) = schem.metadata.preview_image() {
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
