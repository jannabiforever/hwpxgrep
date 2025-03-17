use hwpx::{
    error::HwpxError,
    unzip::{save_extracted, HwpxFile},
};

fn main() -> Result<(), HwpxError> {
    let example_folder = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "examples");
    let example_hwpx = example_folder.clone() + "/example.hwpx";
    let example_result = example_folder.clone() + "/output/";

    let ex = HwpxFile::from_file_path(example_hwpx)?.extract()?;
    save_extracted(&ex, &example_result)?;

    Ok(())
}
