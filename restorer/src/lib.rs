use std::io::Write;

use goblin::pe::optional_header::OptionalHeader;
use goblin::pe::section_table::SectionTable;
use log::{info, trace};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("PE Utils Error: {0}")]
    PEUtils(#[from] pe_utils::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn restore_from_ptr<A: AsRef<str>, B: AsRef<str>>(name: A,
                                                      module_base: usize,
                                                      restored_filename: Option<B>) -> Result<Vec<u8>, Error> {
    let data = unsafe { std::slice::from_raw_parts(module_base as *const u8, 0x1000) };

    let header = pe_utils::parse_headers(data)?;
    trace!("{:#?}", header);
    let optional_headers = pe_utils::get_optional_headers(&header)?;
    let sections = pe_utils::get_sections(&header, data)?;

    let mut vaddr_end: u32 = 0;
    for section in &sections {
        let virtual_end = section.virtual_address + section.size_of_raw_data;
        if virtual_end > vaddr_end {
            vaddr_end = virtual_end;
        }
    }
    let data = unsafe {
        std::slice::from_raw_parts(module_base as *const u8, vaddr_end as usize)
    };
    restore_raw(name, data, optional_headers, &sections, restored_filename)
}

pub fn restore_from_dump<A: AsRef<str>, B: AsRef<str>>(name: A,
                                                       dump: &[u8],
                                                       restored_filename: Option<B>) -> Result<Vec<u8>, Error> {
    let header = pe_utils::parse_headers(dump)?;
    trace!("{:#?}", header);
    let optional_headers = pe_utils::get_optional_headers(&header)?;
    let sections = pe_utils::get_sections(&header, dump)?;
    restore_raw(name, dump, optional_headers, &sections, restored_filename)
}

pub fn restore_raw<A: AsRef<str>, B: AsRef<str>>(name: A,
                                                 dump: &[u8],
                                                 optional_headers: OptionalHeader,
                                                 sections: &[SectionTable],
                                                 restored_filename: Option<B>) -> Result<Vec<u8>, Error> {
    let mut output = vec![0; dump.len()];
    output[0..optional_headers.windows_fields.size_of_headers as usize]
        .copy_from_slice(&dump[0..optional_headers.windows_fields.size_of_headers as usize]);

    let mut eof: u32 = 0;
    for section in sections {
        let phys_end = section.pointer_to_raw_data + section.size_of_raw_data;
        let virtual_end = section.virtual_address + section.size_of_raw_data;
        let virtual_end_aligned = <u32 as pe_utils::MemAlignedAddress<u32>>::get_mem_aligned_address(
            section.virtual_address + section.virtual_size,
            optional_headers.windows_fields.section_alignment,
        );

        trace!(
            "Section name: {}\nPhys ptr: 0x{:02X?}\nPhys size: 0x{:02X?}\nPhys End: 0x{:02X?}\n\
            Virtual ptr: 0x{:02X?}\nVirtual size: 0x{:02X?}\nVirtual End: 0x{:02X?}\n\
            Virtual End Aligned: 0x{:02X?}",
            String::from_utf8_lossy(&section.name),
            section.pointer_to_raw_data,
            section.size_of_raw_data,
            phys_end,
            section.virtual_address,
            section.virtual_size,
            virtual_end,
            virtual_end_aligned
        );

        output[section.pointer_to_raw_data as usize..phys_end as usize]
            .copy_from_slice(&dump[section.virtual_address as usize..virtual_end as usize]);

        if phys_end > eof {
            eof = phys_end;
        }
    }

    match restored_filename {
        None => info!("Since no restored_filename was provided, the restored output will not be saved to a file"),
        Some(filename) => {
            let mut data_file = std::fs::File::create(filename.as_ref())?;
            data_file.write_all(&output[0..eof as usize])?;
            info!("Restored executable saved to: {}", filename.as_ref());
        }
    }

    info!("Executable {} restored successfully", name.as_ref());
    Ok(output)
}