// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
<<<<<<< HEAD
struct Book<'a> {
=======
struct Book <'a>{
>>>>>>> c27de63debe02eaeccfbccc8b25a03a05338e6a9
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
