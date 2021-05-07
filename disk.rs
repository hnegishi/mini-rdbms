
pub struct DiskManager {
  // ヒープファイルのディスクリプた
  heap_file:file,
  // 裁判するページIDを決めるカウンタ
  next_page_id: u64,
}
