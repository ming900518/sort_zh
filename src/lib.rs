//! 這個crate可以協助用戶在Rust專案中更方便的進行中文排序。
//!
//! 在Rust中，如果直接使用`sort()`系列function進行Vec的排序，非ASCII部分的文字會因為Unicode Hex Code的排序而混亂。
//!
//! 本crate提供了`sort_zh()` function 進行正確的排序（預設透過筆畫順序），用戶也可以利用`SortZhOptions`中的設定進行自定義排序。
//!
//! 目前僅支援首字排序，首字重複的情況未實作。

use crate::UpperCaseOrder::*;
use crate::ZhNumberOption::*;
use chinese_number::ChineseVariant::*;
use chinese_number::{parse_chinese_number_to_i64, ChineseNumberCountMethod, ChineseVariant};
use rust_icu_ucol::UCollator;

/// 排序選項
pub struct SortZhOptions {
    /// 繁體中文/簡體中文（預設為繁體中文）
    ///
    /// - 繁體中文使用ICU提供的中文台灣（zh_TW）Collate
    ///
    /// - 簡體中文使用ICU提供的中文中國（zh_CN）Collate
    variant: ChineseVariant,
    /// 中文數字選項（預設為透過筆畫排序）
    zh_number_option: ZhNumberOption,
}

/// 中文數字選項
#[derive(PartialEq)]
pub enum ZhNumberOption {
    /// 透過ICU預設方式排序
    ICUDefault,
    /// 透過中文含義排序（不排序大寫數字）
    Definition,
    /// 透過中文含義排序（排序大寫數字，需指定大寫數字排序）
    DefinitionWithUpperCase(UpperCaseOrder),
}

/// 大寫數字排序選項
#[derive(PartialEq)]
pub enum UpperCaseOrder {
    // 大寫數字排於小寫數字之前（例：`["壹", "貳", "一", "二"]`）
    Before,
    // 大寫數字排於小寫數字之後（例：`["一", "二", "壹", "貳"]`）
    After,
}

impl Default for SortZhOptions {
    fn default() -> Self {
        SortZhOptions {
            variant: Traditional,
            zh_number_option: ZhNumberOption::default(),
        }
    }
}

impl Default for ZhNumberOption {
    fn default() -> Self {
        ICUDefault
    }
}

impl Default for UpperCaseOrder {
    fn default() -> Self {
        After
    }
}

static LOWERCASE_NUM: [char; 50] = [
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百', '千', '萬', '億', '兆',
    '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極', '零', '一', '二', '三', '四', '五', '六',
    '七', '八', '九', '十', '百', '千', '万', '亿', '兆', '京', '垓', '秭', '穰', '沟', '涧', '正',
    '载', '极',
];

static UPPERCASE_NUM: [char; 50] = [
    '零', '壹', '貳', '參', '肆', '伍', '陸', '柒', '捌', '玖', '拾', '佰', '仟', '萬', '億', '兆',
    '京', '垓', '秭', '穰', '溝', '澗', '正', '載', '極', '零', '壹', '贰', '参', '肆', '伍', '陆',
    '柒', '捌', '玖', '拾', '佰', '仟', '万', '亿', '兆', '京', '垓', '秭', '穰', '沟', '涧', '正',
    '载', '极',
];

pub trait SortZh {
    /// 中文排序
    fn sort_zh(&self, options: SortZhOptions) -> Vec<&str>;
}

impl SortZh for Vec<&str> {
    fn sort_zh(&self, options: SortZhOptions) -> Vec<&str> {
        let collator = match options.variant {
            Traditional => UCollator::try_from("zh-TW"),
            Simple => UCollator::try_from("zh-CN"),
        }
        .expect("Could not make collator.");

        let orig_vec = self.clone();
        let mut ascii_word_vec: Vec<(usize, &str)> = Vec::new();
        let mut zh_upper_number_vec: Vec<(usize, i64)> = Vec::new();
        let mut zh_lower_number_vec: Vec<(usize, i64)> = Vec::new();
        let mut zh_word_vec: Vec<(usize, &str)> = Vec::new();

        self.iter().enumerate().for_each(|(i, element)| {
            let char = element.chars().next().unwrap();
            if char.is_ascii() {
                ascii_word_vec.push((i, element))
            } else {
                let zh_number_option = &options.zh_number_option;
                match zh_number_option {
                    ICUDefault => zh_word_vec.push((i, element)),
                    Definition | DefinitionWithUpperCase(_) => {
                        match parse_chinese_number_to_i64(
                            ChineseNumberCountMethod::TenThousand,
                            element,
                        ) {
                            Ok(parsed) => {
                                if zh_number_option == &DefinitionWithUpperCase(Before)
                                    || zh_number_option == &DefinitionWithUpperCase(After)
                                {
                                    if LOWERCASE_NUM.contains(&char) {
                                        zh_lower_number_vec.push((i, parsed))
                                    } else if UPPERCASE_NUM.contains(&char) {
                                        zh_upper_number_vec.push((i, parsed))
                                    } else {
                                        zh_word_vec.push((i, element))
                                    }
                                } else {
                                    zh_lower_number_vec.push((i, parsed))
                                }
                            }
                            Err(_) => zh_word_vec.push((i, element)),
                        }
                    }
                }
            }
        });

        let mut final_vec = sort_ascii_word(ascii_word_vec);
        final_vec.append(&mut sort_zh_number(
            zh_upper_number_vec,
            zh_lower_number_vec,
            options.zh_number_option
        ));
        final_vec.append(&mut sort_zh_word(zh_word_vec, collator));

        final_vec.into_iter().map(|i| {
            orig_vec[i]
        }).collect::<Vec<&str>>()
    }
}

fn sort_ascii_word(mut ascii_word_vec: Vec<(usize, &str)>) -> Vec<usize> {
    ascii_word_vec.sort_by(|(_, a), (_, b)| a.cmp(b));
    let (processed_ascii_word, _): (Vec<usize>, Vec<_>) = ascii_word_vec.into_iter().unzip();
    processed_ascii_word
}

fn sort_zh_number(
    mut zh_upper_number_vec: Vec<(usize, i64)>,
    mut zh_lower_number_vec: Vec<(usize, i64)>,
    zh_number_option: ZhNumberOption
) -> Vec<usize> {
    zh_upper_number_vec.sort_by(|(_, a_value), (_, b_value)| a_value.cmp(b_value));
    zh_lower_number_vec.sort_by(|(_, a_value), (_, b_value)| a_value.cmp(b_value));
    let (mut zh_upper_number_vec, _): (Vec<usize>, Vec<_>) = zh_upper_number_vec.into_iter().unzip();
    let (mut zh_lower_number_vec, _): (Vec<usize>, Vec<_>) = zh_lower_number_vec.into_iter().unzip();
    match zh_number_option {
        DefinitionWithUpperCase(upper_case_order) => {
            match upper_case_order {
                Before => {
                    zh_upper_number_vec.append(&mut zh_lower_number_vec);
                    zh_upper_number_vec
                },
                After => {
                    zh_lower_number_vec.append(&mut zh_upper_number_vec);
                    zh_lower_number_vec
                }
            }
        },
        _ => zh_lower_number_vec
    }
}

fn sort_zh_word(mut zh_word_vec: Vec<(usize, &str)>, collator: UCollator) -> Vec<usize> {
    zh_word_vec.sort_by(|(_, a_value), (_, b_value)| {
        collator.strcoll_utf8(a_value, b_value).expect("Failed to collate with collator.")
    });
    let (index_vec, _): (Vec<usize>, Vec<_>) = zh_word_vec.into_iter().unzip();
    index_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icu_default() {
        let test = vec!["肆", "1", "一", "2", "二", "參", "正"];
        println!("Testing sort by ICU default collate, with test data: {:?}...", test);
        let result = test.sort_zh(SortZhOptions::default());
        println!("Result: {:?}", result);
        assert_eq!(result, vec!["1", "2", "一", "二", "正", "參", "肆"]);
    }

    #[test]
    fn definition() {
        let test = vec!["肆", "1", "一", "2", "二", "參", "正"];
        println!("Testing sort by definition, with test data: {:?}...", test);
        let result = test.sort_zh(SortZhOptions{
            variant: Traditional,
            zh_number_option: Definition,
        });
        println!("Result: {:?}", result);
        assert_eq!(result, vec!["1", "2", "一", "二", "參", "肆", "正"]);
    }

    #[test]
    fn definition_with_upper_case_before() {
        let test = vec!["肆", "1", "一", "2", "二", "參", "正"];
        println!("Testing sort by definition with upper case number placed before lower case number, with test data: {:?}...", test);
        let result = test.sort_zh(SortZhOptions{
            variant: Traditional,
            zh_number_option: DefinitionWithUpperCase(Before),
        });
        println!("Result: {:?}", result);
        assert_eq!(result, vec!["1", "2", "參", "肆", "一", "二", "正"]);
    }

    #[test]
    fn definition_with_upper_case_after() {
        let test = vec!["肆", "1", "一", "2", "二", "參", "正"];
        println!("Testing sort by definition with lower case number placed before upper case number,, with test data: {:?}...", test);
        let result = test.sort_zh(SortZhOptions{
            variant: Traditional,
            zh_number_option: DefinitionWithUpperCase(After),
        });
        println!("Result: {:?}", result);
        assert_eq!(result, vec!["1", "2", "一", "二", "參", "肆", "正"]);
    }

}
