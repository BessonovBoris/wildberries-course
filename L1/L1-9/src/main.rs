fn main() {
    let num = 4i64;

    println!("{}", bit_to_1(num, 2, false));
}

fn bit_to_1(num: i64, i: usize, to_one: bool) -> i64 {
    if to_one {
        let tmp = 1 << i;
        let ans = num | tmp;

        return ans
    }

    let tmp = i64::MAX - (1 << i);
    let ans = num & tmp;

    ans
}