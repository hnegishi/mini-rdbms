// 構造体定義(new type pattern)
pub struct PageId(pub u64);

pub struct DiskManager {
  // ヒープファイルのディスクリプた
  heap_file:file,
  // 裁判するページIDを決めるカウンタ
  next_page_id: u64,
}

// impl メソッド実装のブロック
// io::Result<self>
// io::Result ファイルが見つからないorディスクがいっぱい等のI/O関連のエラーが起こりうる
// <self> 成功した場合の値
impl DiskManager {
  // constracter
  pub fn new(date_file: File) -> io::Result<Self> {
    // ファイルサイズ取得
    let heap_file_size = heap_file.metadata()?.len();
    // 次に裁判するブロック単位のIDを計算
    let next_page_id = heap_file_size / PAGE_SIZE as u64;
    Ok(Self{
      heap_file,
      next_page_id,
    })
  }

  // open heap file
  //pathを指定して開く
  pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
    let heap_file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(heap_file_path)?; // ?はエラーが返った時の早期return。
    Self::new(heap_file) // DiskManager::open -self-> DiskManager::new
  }

  // read page data
  // &self &mut self はレシーバ
  pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {

  }

  // write page data
  pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {

  }

// incrementして値を返してるだけ
  pub fn allocate_page(&mut self) -> PageId {
    let page_id = self.next_page_id;
    self.next_page_id +=1;
    PageId(page_id)
  }
}
