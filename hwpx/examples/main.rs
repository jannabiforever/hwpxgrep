use hwpx::unzip;

fn main() -> anyhow::Result<()> {
    let example_folder = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "examples");
    let example_hwpx = example_folder.clone() + "/example.hwpx";
    let example_result = example_folder.clone() + "/output/";

    unzip::HwpxFile::from_file_path(example_hwpx)?
        .extract()?
        .save(example_result)?;

    Ok(())
}
