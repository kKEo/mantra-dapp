
struct Book {
    title: String,
    author: String,
    year_published: u16,
    is_available: bool,
}


fn title(b: &Book) -> String {
  b.title.clone()
}


fn add(a: i32, b: i32) -> i32 {
  a + b
}

pub fn main() {
    let res = add(5, 4);
    println!("The sum: {}", res);

    let my_book: Book = Book {
      title: String::from("The Hitchhiker's Guide to the Galaxy"),
      author: String::from("Douglas Adams"),
      year_published: 1979,
      is_available: true,
    };

    println!("The book title: {}", title(&my_book));
}
