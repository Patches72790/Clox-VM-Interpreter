use rox::chunk::Chunk;

fn main() {
    let mut my_c = Chunk::new();
    my_c.write_chunk(8);

    println!("{:?}", my_c);
}
