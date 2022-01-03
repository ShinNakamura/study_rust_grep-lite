fn main() {
    let search_item = "picture";
    let quote = "\
Everything is a picture.
Something else.";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_item) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

    list_lesson();
}

fn list_lesson() {
    let l1 = [1, 2, 3];
    let l2: [u8; 3] = [4, 5, 6];
    let zeros: [u8; 3] = [0; 3]; // init all 0

    let arrs = [l1, l2, zeros];

    for a in &arrs {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{}", n);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\tsum: {}", sum);
    }
}