use circular_reader::CircularReader;

fn main() {
    let vec = vec!(1, 2, 3, 4, 5);
    let mut cr = CircularReader::new(vec);
    for idx in 0..10 {
        println!("{}: {}", idx, cr.next().unwrap());
    }
}
