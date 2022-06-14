use core::option::Option;
use core::option::Option::Some;
use stivale_boot::v2::StivaleStruct;
use tar_no_std::TarArchiveRef;

// Gets the start address of the first module
// Returns None if no modules where fed in
pub fn get_initrd_address(boot_info: &StivaleStruct) -> Option<*const u8> {
    let modules = boot_info.modules()?;
    let module = modules.iter().next()?;
    Some(module.start as *const u8)
    
}

// Gets the size of the first module
// Returns None if no modules where fed in
pub fn get_initrd_len(boot_info: &StivaleStruct) -> Option<usize> {
    let modules = boot_info.modules()?;
    let module = modules.iter().next()?;
    Some((module.end - module.start) as usize)
}

// Gets the first module as a byte slice which is a little bit easier to work with
// Returns None if no modules where passed into the kernel
pub fn get_initrd_as_byte_slice(boot_info: &StivaleStruct) -> Option<&[u8]>{
    let address = get_initrd_address(boot_info)?;
    let len = get_initrd_len(boot_info)?;
    unsafe {
        Some(core::slice::from_raw_parts(address, len))
    }
}

pub fn get_initrd<'a>(boot_info: &'static StivaleStruct) -> TarArchiveRef<'a> {
    TarArchiveRef::<'a>::new(
        get_initrd_as_byte_slice(boot_info)
            .expect("A initrd is expected!")
    )
}
