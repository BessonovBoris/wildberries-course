pub fn unpacking(input: &String) -> Result<String, String> {
    let mut answer = String::new();

    let mut i = 0;
    while i < input.len() {
        let mut escaped = false;

        if input.chars().nth(i).unwrap() == '\\' {
            i += 1;
            if i == input.len() {
                return Err(String::from("wrong format"));
            }

            escaped = true;
        }
        let char = input.chars().nth(i).unwrap();

        if char.is_digit(10) && !escaped {
            return Err(String::from("wrong format"));
        }

        let mut j = i + 1;
        while j < input.len() {
            let char = input.chars().nth(j).unwrap();
            if !char.is_digit(10) {
                break;
            }
            j += 1;
        }

        let num = input.get(i + 1..j).unwrap();
        i = j;
        if !num.is_empty() {
            for _ in 0..num.parse::<i32>().unwrap() {
                answer.push(char);
            }
        } else {
            answer.push(char);
        }
    }

    Ok(answer)
}
