/**
 * Move all zero to begining
 */
fn main() {
    let mut facts = [0, 1, 3, 0, -3, 4, 9, 0, 0, 2, 1, 0, 5];

    // Move all none zero to the end of array
    let mut none_zero = facts.len() - 1;
    for i in (0..facts.len()).rev() {
        if facts[i] != 0 {
            facts[none_zero] = facts[i];
            none_zero -= 1;
        }
    }

    // fill zero at the beginning
    for i in 0..=none_zero {
        facts[i] = 0;
    }

    println!("{:?}", facts);
}
