use md5::{Digest, Md5};

fn main() {
    // process input message
    let input = "iwrupvqb";
    let mut i = 0;
    loop {
        let input = format!("{}{}", input, i);

        let mut hasher = Md5::new();
        hasher.update(input);

        // acquire hash digest in the form of GenericArray,
        // which in this case is equivalent to [u8; 16]
        let result = hasher.finalize();
        // cast the array to hexadecimal
        let t = format!("{:x}", result);
        if t.starts_with("000000") {
            print!("{} -> {}", i, t);
            break;
        }
        i += 1;
    }
}
