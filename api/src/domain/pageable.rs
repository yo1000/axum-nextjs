use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidateLength};

fn default_page() -> i32 { 0 }
fn default_size() -> i32 { 20 }

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Validate)]
pub struct Pageable {
    #[serde(default = "default_page", deserialize_with = "deserialize_i32")]
    #[validate(range(min = 0))]
    pub page: i32,
    #[serde(default = "default_size", deserialize_with = "deserialize_i32")]
    #[validate(range(min = 1, max = 1000))]
    pub size: i32,
}

impl Pageable {
    pub fn of(
        page: i32,
        size: i32,
    ) -> Self {
        Self {
            page,
            size,
        }
    }

    pub fn offset(&self) -> i64 {
        self.page as i64 * self.size as i64
    }
}

fn deserialize_i32<'de, D: Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i32),
    }

    match StringOrInt::deserialize(d)? {
        StringOrInt::String(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
        StringOrInt::Int(i)    => Ok(i),
    }
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
    pub total_elements: i64,
    pub number: i32,
    pub size: i32,
    pub total_pages: i32,
    pub number_of_elements: i32,
    pub empty: bool,
    pub first: bool,
    pub last: bool,
}

impl <T> Page<T> {
    pub fn new(
        content: Vec<T>,
        pageable: &Pageable,
        total_elements: i64
    ) -> Self {
        let max_page =
            if total_elements == 0 { 0 }
            else { ((total_elements - 1) / pageable.size as i64) as i32 };

        let number =
            if pageable.page <= max_page { pageable.page }
            else { max_page };

        let first = number == 0;
        let last = number == max_page;

        let content_length = content.length().unwrap_or_else(|| 0);

        Self {
            content,
            pageable: *pageable,
            total_elements,
            number,
            size: pageable.size,
            total_pages: max_page,
            number_of_elements: content_length as i32,
            empty: total_elements == 0,
            first,
            last,
        }
    }
}

#[test]
fn test_page_number() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(0, 10), 200);
    assert_eq!(page.number, 0);
}

#[test]
fn test_page_number_just_page() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(100, 10), 200);
    assert_eq!(page.number, 19);
}

#[test]
fn test_page_number_under_page() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(100, 10), 199);
    assert_eq!(page.number, 19);
}

#[test]
fn test_page_number_over_page() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(100, 10), 201);
    assert_eq!(page.number, 20);
}

#[test]
fn test_page_number_over2_page() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(100, 10), 202);
    assert_eq!(page.number, 20);
}

#[test]
fn test_page_number_empty_page() {
    let contents: Vec<String> = Vec::new();

    let page = Page::new(contents, &Pageable::of(100, 10), 0);
    assert_eq!(page.number, 0);
}

#[test]
fn test_page_is_first() {
    let contents: Vec<String> = Vec::new();

    let page = Page::new(contents, &Pageable::of(0, 10), 0);
    assert_eq!(page.first, true);
}

#[test]
fn test_page_is_first_false() {
    let mut contents = Vec::new();
    for i in 0..10 {
        contents.push(format!("test-{i}"));
    }

    let page = Page::new(contents, &Pageable::of(1, 10), 200);
    assert_eq!(page.first, false);
}

#[test]
fn test_page_is_first_empty_page() {
    let contents: Vec<String> = Vec::new();

    let page = Page::new(contents, &Pageable::of(1, 10), 0);
    assert_eq!(page.first, true);
}
