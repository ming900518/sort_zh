# sort_zh
這個crate可以協助用戶在Rust專案中更方便的進行中文排序。

在Rust中，如果直接使用`sort()`系列function進行Vec的排序，非ASCII部分的文字會因為Unicode Hex Code的排序而混亂。

本crate提供了`sort_zh()` function 進行正確的排序（預設透過筆畫順序），用戶也可以利用`SortZhOptions`中的設定進行自定義排序。

目前僅支援首字排序，首字重複的情況未實作。
## 測試結果
```console
$ cargo test -- --nocapture
   Compiling sort_zh v0.1.0 (/home/root/GitHub/sort_zh)
    Finished test [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests src/lib.rs (target/debug/deps/sort_zh-c61a611dc8e5c8cf)

running 4 tests
Testing sort by ICU default collate, with test data: ["肆", "1", "一", "2", "二", "參", "正"]...
Testing sort by definition, with test data: ["肆", "1", "一", "2", "二", "參", "正"]...
Testing sort by definition with lower case number placed before upper case number,, with test data: ["肆", "1", "一", "2", "二", "參", "正"]...
Testing sort by definition with upper case number placed before lower case number, with test data: ["肆", "1", "一", "2", "二", "參", "正"]...
Result: ["1", "2", "一", "二", "正", "參", "肆"]
Result: ["1", "2", "一", "二", "參", "肆", "正"]
Result: ["1", "2", "一", "二", "參", "肆", "正"]
Result: ["1", "2", "參", "肆", "一", "二", "正"]
test tests::icu_default ... ok
test tests::definition ... ok
test tests::definition_with_upper_case_after ... ok
test tests::definition_with_upper_case_before ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sort_zh

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
