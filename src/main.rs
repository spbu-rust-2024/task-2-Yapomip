
// abcddcba
//         

fn all_calculate(input_str: &[u8]) -> &[u8] {
    let (mut ans_len, mut ans_r, mut ans_l) = (0, 0, 0);
    
    // odd palindromes
    let mut d1: Vec<usize> = vec![0; input_str.len()];
    let mut l1 = 0;
    let mut r1 = 0;
    
    // even palindromes
    let mut d2: Vec<usize> = vec![0; input_str.len()];
    let mut l2 = 0;
    let mut r2 = 0;
    
    let mut len;
    
    for i in 0..input_str.len() {
        // odd palindromes
        len = 0;
        
        if i <= r1 && r1 != 0 {
            len = std::cmp::min(r1 - i, d1[r1 - i + l1]);
        }
        while i + len + 1 < input_str.len() && i >= len + 1 && input_str[i + len + 1] == input_str[i - len - 1] {
            len += 1;
        }
        d1[i] = len;
        if i + len > r1 {
            l1 = i - len;
            r1 = i + len;
        }
        
        if len * 2 + 1 > ans_len {
            ans_len = len * 2 + 1;
            ans_r = r1;
            ans_l = l1;
        }
        
        // even palindromes
        len = 0;
        
        if i <= r2 && r2 != 0 {
            len = std::cmp::min(r2 - i + 1, d2[r2 - i + l2 + 1]);
        }
        while i + len < input_str.len() && i >= len + 1 && input_str[i + len] == input_str[i - len - 1] {
            len += 1;
        }
        d2[i] = len;
        
        if i + len > r2 + 1 {
            l2 = i - len;
            r2 = i + len - 1;
        }
        if len * 2 > ans_len {
            ans_len = len * 2;
            ans_r = r2;
            ans_l = l2;
        }
    }

    return &input_str[ans_l..=ans_r];
}

fn max_palindrom(input_str: &str) -> &str {
     return std::str::from_utf8(all_calculate(input_str.as_bytes())).unwrap();
}

fn main() {
    let mut input_str = String::new();
    
    std::io::stdin().read_line(&mut input_str).expect("Fail to read line");

    println!("{}", max_palindrom(&input_str));
}
