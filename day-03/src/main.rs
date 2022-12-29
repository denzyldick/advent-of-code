use std::fs;
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let mut sum = 0;
    for (i, v) in input.lines().enumerate() {
        let splitted = v.split_at(v.len() / 2);
        let mut duplicate = Vec::new();
        for c in splitted.0.chars().into_iter() {
            for q in splitted.1.chars().into_iter() {
                if q == c {
                    let is_upper = q.is_uppercase();
                    let z = q.to_lowercase().into_iter().next().unwrap();

                    let mut o = ASCII_LOWER.iter().position(|&r| r == z).unwrap() + 1;
                    if is_upper {
                        o = o + 26 
                    }
                    duplicate.push(o)
                }
            }
        }
        duplicate.dedup();
        for e in duplicate.iter().enumerate() {
            let n = e.1;
            sum = sum + n;
        }
         }
   println!("{:?}", sum)

}
