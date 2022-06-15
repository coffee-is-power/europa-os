use alloc::{slice, vec};
use alloc::vec::Vec;
use acpi::mcfg::{Mcfg};
use core::mem;
use core::ptr::NonNull;
use acpi::{AcpiError, AcpiTable, AcpiTables};
use rsdp::handler::{AcpiHandler, PhysicalMapping};

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct McfgEntry {
    pub base_address: u64,
    pub pci_segment_group: u16,
    pub bus_number_start: u8,
    pub bus_number_end: u8,
    _reserved: u32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct PCIDeviceHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}
pub fn entries(mcfg: &Mcfg) -> &[McfgEntry] {
    let length = mcfg.header().length as usize - mem::size_of::<Mcfg>();

    // Intentionally round down in case length isn't an exact multiple of McfgEntry size
    // (see rust-osdev/acpi#58)
    let num_entries = length / mem::size_of::<McfgEntry>();

    unsafe {
        let pointer = (mcfg as *const Mcfg as *const u8).add(mem::size_of::<Mcfg>()) as *const McfgEntry;
        slice::from_raw_parts(pointer, num_entries)
    }
}

/// Describes a set of regions of physical memory used to access the PCIe configuration space. A
/// region is created for each entry in the MCFG. Given the segment group, bus, device number, and
/// function of a PCIe device, the `physical_address` method on this will give you the physical
/// address of the start of that device function's configuration space (each function has 4096
/// bytes of configuration space in PCIe).
#[derive(Clone, Debug)]
pub struct PciConfigRegions {
    regions: Vec<McfgEntry>,
}

impl PciConfigRegions {
    pub fn new<H>(tables: &AcpiTables<H>) -> Result<PciConfigRegions, AcpiError>
        where
            H: AcpiHandler,
    {
        let mcfg = unsafe {
            tables
                .get_sdt::<Mcfg>(acpi::sdt::Signature::MCFG)?
                .ok_or(AcpiError::TableMissing(acpi::sdt::Signature::MCFG))?
        };
        Ok(Self { regions: entries(&mcfg).iter().copied().collect() })
    }

    /// Get the physical address of the start of the configuration space for a given PCIe device
    /// function. Returns `None` if there isn't an entry in the MCFG that manages that device.
    pub fn physical_address(&self, bus: u8, device: u8, function: u8) -> Option<u64> {
        // First, find the memory region that handles this segment and bus. This method is fine
        // because there should only be one region that handles each segment group + bus
        // combination.
        let region = self.regions.iter().find(|region| {
            (region.bus_number_start..=region.bus_number_end).contains(&bus)
        })?;

        Some(
            region.base_address
                + ((u64::from(bus - region.bus_number_start) << 20)
                | (u64::from(device) << 15)
                | (u64::from(function) << 12)),
        )
    }
    /*
        Returns the PCIDeviceHeader of all *valid* functions
    */
    pub fn get_pci_functions(&self) -> Vec<&PCIDeviceHeader>{
        let mut result: Vec<& PCIDeviceHeader> = vec![];
        for region in &self.regions {
            for bus in region.bus_number_start..=region.bus_number_end {
                for device in 0..32 {
                    for function in 0..8 {unsafe {
                        let header = &*(self.physical_address(bus, device, function).unwrap() as *const PCIDeviceHeader);
                        if header.device_id == 0 || header.device_id == 0xFFFF {
                            continue;
                        }
                        result.push(header);
                    }}
                }
            }
        }
        result
    }
}

#[derive(Clone)]
pub struct AcpiHandlerImpl;

impl AcpiHandler for AcpiHandlerImpl {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        return PhysicalMapping::new(physical_address, NonNull::new(physical_address as *mut T).unwrap(), size, size, AcpiHandlerImpl);
    }

    fn unmap_physical_region<T>(_: &PhysicalMapping<Self, T>) {}
}
pub fn get_pci_config_regions(rsdp: u64) -> Option<PciConfigRegions>{
    let tables = unsafe{
        AcpiTables::from_rsdp(AcpiHandlerImpl, rsdp as usize).ok()?
    };
    PciConfigRegions::new(&tables).ok()
}