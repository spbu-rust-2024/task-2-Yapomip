
// abcddcba
//         

fn calculate1(input_str: &[u8]) -> &[u8] {
    let (mut ans_len, mut ans_r, mut ans_l) = (0, 0, 0);

    let mut d1: Vec<usize> = vec![0; input_str.len()];
    let mut l = 0;
    let mut r = 0;

    for i in 0..input_str.len() {
        let mut len = 0;
        if i <= r && r != 0 {
            len = std::cmp::min(r - i, d1[r - i + l]);
        }
        while i + len + 1 < input_str.len() && i >= len + 1 && input_str[i + len + 1] == input_str[i - len - 1] {
            len += 1;
        }
        d1[i] = len;
        if i + len > r {
            l = i - len;
            r = i + len;
        }
        if len > ans_len {
            ans_len = len;
            ans_r = r;
            ans_l = l;
        }
    }

    return &input_str[ans_l..=ans_r];
}

fn calculate2(input_str: &[u8]) -> &[u8] {
    let (mut ans_len, mut ans_r, mut ans_l) = (0, 0, 0);

    let mut d2: Vec<usize> = vec![0; input_str.len()];
    let mut l = 0;
    let mut r = 0;
    
    for i in 1..input_str.len() {
        let mut len = 0;
        if i <= r && r != 0 {
            len = std::cmp::min(r - i + 1, d2[r - i + l + 1]);
        }
        while i + len < input_str.len() && i >= len + 1 && input_str[i + len] == input_str[i - len - 1] {
            len += 1;
        }
        d2[i] = len;
        
        if i + len - 1 > r {
            l = i - len;
            r = i + len - 1;
        }
        if len > ans_len {
            ans_len = len;
            ans_r = r;
            ans_l = l;
        }
    }

    return &input_str[ans_l..=ans_r];
}

fn max_palindrom(input_str: &str) -> &[u8] {
    let ans1 = calculate1(input_str.as_bytes());
    let ans2 = calculate2(input_str.as_bytes());

    if ans1.len() > ans2.len() {
        ans1
    } else {
        ans2
    }
}

fn main() {
    let mut input_str = String::new();

    std::io::stdin().read_line(&mut input_str).expect("Fail to read line");

    let palindrom = max_palindrom(&input_str);
    println!("{}", std::str::from_utf8(&palindrom).unwrap());
}
