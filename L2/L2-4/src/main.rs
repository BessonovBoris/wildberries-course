use L2_4::search_anagrams;

fn main() {
    let dictionary = ["abv", "lb", "lyr", "LYR", "lla", "lll"];
    let anagrams = search_anagrams(&dictionary);

    for anagram in anagrams.iter() {
        println!("{:?}", anagram);
    }

    dictionary.iter().for_each(|d| {
        println!("{}", d);
    })
}
