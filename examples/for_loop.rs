use circular_reader::CircularReader;

fn main() {
    let mut count = 0;
    let buf = vec!(1, 2, 3, 4, 5);
    let buf_len = buf.len();
    for x in CircularReader::new(buf) {
        println!("{}", x);
        count += 1;
        if count >= buf_len * 3 {
            break;
        } else if count % buf_len == 0 {
            println!("");
        }
    }
}
