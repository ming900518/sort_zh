# sort_zh
這個crate可以協助用戶在Rust專案中更方便的進行中文排序。

在Rust中，如果直接使用 `sort()` 系列function進行Vec的排序，非ASCII部分的文字會因為Unicode Hex Code的排序而混亂。

本crate提供了 `sort_zh()` function 進行正確的排序（預設透過筆畫順序），用戶也可以利用 `SortZhOptions` 中的設定進行自定義排序。

內附單元測試，可透過 `cargo test` 進行測試。
