use recursion_optimization::foo2;

fn main() {
    let mut sum = 0usize;
    for x in 0 .. 100 {
        for y in 0 .. 100 {
            sum += foo2(x % 100, y % 100) as usize;
        }
    }
    println!("{}", sum);
}