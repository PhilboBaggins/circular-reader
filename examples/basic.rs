use circular_reader::CircularReader;

fn main() {
    let buf = vec![1, 2, 3, 4, 5];
    let mut cr = CircularReader::new(buf);
    for idx in 0..10 {
        println!("{}: {}", idx, cr.next().unwrap());
    }
}
