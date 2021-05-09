
pub struct DiskManager {
  // ヒープファイルのディスクリプた
  heap_file:file,
  // 裁判するページIDを決めるカウンタ
  next_page_id: u64,
}

impl DiskManager {
  // constracter
  pub fn new(date_file: File) -> io::Result<Self> {

  }
}

// open file path
pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {

}

// read page data
pub fn read_page_data(&mul self, page_id: PageId, data: mut [u8]) -> io::Result<()> {

}

// write page data
pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {

}
