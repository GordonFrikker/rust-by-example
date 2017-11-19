use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Потому что в противном случае мы должны были бы выразить это как
// `T: Debug` или использовать другой метод косвенного подхода,
// для этого требуется утверждение `where`:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // Мы хотим `Option<T>: Debug` как наше ограничение типажа,
    // потому то это то, что печатается. В противном случае 
    // использовалось бы неправильное ограничение типажа.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
