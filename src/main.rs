use std::fs::read_to_string;

const VARIANT: usize = 3; // 4 -1
const BITTNESS: usize = 16;

fn main() {
    let hadamard = generate_hadamard_mx(16);
    let data = read_data("DataBinary.dat");
    let result = decipher(&data, &hadamard[VARIANT]);
    println!(
        "{}",
        result
            .iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join("")
    );
}

/// Generates Hadamard matrix of size `n`
fn generate_hadamard_mx(n: usize) -> Vec<Vec<i8>> {
    match n {
        1 => vec![vec![1]],
        _ => {
            let mut res = generate_hadamard_mx(n / 2);
            res.iter_mut().for_each(|v| *v = [v, &v[0..n / 2]].concat());
            for i in 0..(n / 2) {
                res.push(res[i].clone());
                res[n / 2 + i][0..n / 2].iter_mut().for_each(|i| {
                    *i = -*i;
                });
            }
            res
        }
    }
}

/// Reads ints from file
fn read_data(path: &str) -> Vec<i8> {
    read_to_string(path)
        .expect("Failed to read")
        .trim()
        .split(|c: char| c.is_whitespace() || c == '\n')
        .filter_map(|s| match s {
            "" => None,
            _ => Some(s.parse::<i8>().unwrap()),
        })
        .collect()
}

/// Decipher messages from input
fn decipher(data: &[i8], hadamrd_by_variant: &[i8]) -> Vec<bool> {
    data.chunks(BITTNESS)
        .collect::<Vec<_>>()
        .iter()
        .map(|chunk| -> bool {
            let sum = chunk.iter().enumerate().fold(0_i32, |sum, item| {
                sum + *item.1 as i32 * hadamrd_by_variant[item.0] as i32
            });
            sum / BITTNESS as i32 > 0
        })
        .collect::<Vec<bool>>()
}
