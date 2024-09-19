use std::collections::HashSet;
use std::hash::Hash;

// use rust realisation
fn intersection_v1<T: Clone>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T>
where T: Eq, T: Hash {
    a.intersection(&b).cloned().collect()
}

// my realisation
fn intersection_v2<T>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T>
where T: Eq, T: Hash {
    let mut ans = HashSet::new();

    for i in a {
        if b.contains(&i) {
            ans.insert(i);
        }
    }

    ans
}

fn main() {
    let a = HashSet::from_iter(vec![1, 2, 3]);
    let b = HashSet::from_iter(vec![2, 3, 4, 5]);

    // let c = intersection_v1(a, b);
    let c = intersection_v2(a, b);

    println!("{:?}", c);
}
