use exe::{Buffer, CCharString, PE, SectionCharacteristics, VecPE};
use std::path::Path;

pub fn check_section_headers(path: impl AsRef<Path>) -> Result<bool, Box<dyn std::error::Error>> {
    let image = VecPE::from_disk_file(&path)?;
    let table = image.get_section_table()?;

    let mut retplne = false;
    let mut voltbl = false;
    for h in table {
        let header_is_read = (h.characteristics & SectionCharacteristics::MEM_READ)
            != SectionCharacteristics::empty();
        match h.name.as_str()? {
            ".retplne" => retplne = header_is_read,
            ".voltbl" => voltbl = header_is_read,
            _ => continue,
        }
    }

    Ok(retplne && voltbl)
}

pub fn patch_section_headers(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut image = VecPE::from_disk_file(&path)?;
    let table = image.get_mut_section_table()?;

    for h in table {
        if matches!(h.name.as_str()?, ".retplne" | ".voltbl") {
            h.characteristics
                .set(SectionCharacteristics::MEM_READ, true);
        }
    }

    image.get_buffer().save(path)?;
    Ok(())
}
