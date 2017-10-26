struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// Эти функции действительны только для типов реализующих эти типажи.
// То, что типажи пусты, не имеет значения.
fn red<T: Red>(_: &T)   -> &'static str { "красная" }
fn blue<T: Blue>(_: &T) -> &'static str { "синяя" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` не будет работать для blue_jay, ни наоборот,
    // из-за ограничений.
    println!("Кардинал {} птица", red(&cardinal));
    println!("Голубая сойка {} птица", blue(&blue_jay));
    //println!("Индюк {} птица", red(&_turkey));
    // ^ TODO: Попробуйте раскомментировать эту строку.
}
