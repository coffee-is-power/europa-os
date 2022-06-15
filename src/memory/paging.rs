pub use x86_64::structures::paging::PageTable;

#[allow(dead_code)]
pub unsafe fn get_current_page_table()
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let page_table_ptr = phys.as_u64() as *mut PageTable;

    &mut *page_table_ptr
}