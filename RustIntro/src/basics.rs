/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0 {
        -1
    } else {
        (n+1)*n/2
    }
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for i in ls.iter() {
        if i <= &e && i >= &s {
            count += 1;
        }
    }
    count
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    for i in target.iter() {
        if !set.contains(i) {
            return false
        }
    }
    true
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    let size: f64 = ls.len() as f64;
    if size == 0.0 {
        None
    } else {
        let mut sum: f64 = 0.0;
        for i in ls.iter() {
            sum += i;
        }
        Some(sum/size)
    }
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    let mut ans = 0;
    for i in 0..ls.len() {
      ans+= &ls[ls.len()-1-i] * 2_i32.pow((i) as u32) as i32
    }
    ans
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = Vec::new();
    let mut factors2: Vec<u32> = Vec::new();
    factors.push(n);

    while let Some(f) = /*n*/ factors.pop(){
      if f == 2{
        factors2.push(f);
      }
      else{
        for i in 2..f{
            if f%i == 0{
                factors.push(i);
                factors.push(f/i);
                break;
            }
            if i == f-1{
                factors2.push(f);
            }
        }
      }
    }

    factors2.sort();
    factors2
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    if lst.len() == 0 {
        return ans;
    }
    for i in 1..lst.len(){
        ans.push(lst[i]);
    }
    ans.push(lst[0]);
    ans
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    if s.len() < target.len() {
        return false
    }
    for i in 0..s.len()-target.len()+1 {
        if &s[i..i+target.len()] == target{
            return true
        }
    }
    false
}


/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.len() == 0 { 
        return None 
    }
    
    let mut curr = s.chars().nth(0); 
    let mut temp_count = 0; 
    let mut temp_start = 0; 
    let mut count = 0; 
    let mut start = 0; 

    for i in 0..s.len() {
        let c = s.chars().nth(i);
        if c == curr { 
            temp_count += 1; 
        } else {
            if temp_count > count {
                count = temp_count;
                start = temp_start;
            }
            curr = c.clone();
            temp_count = 1;
            temp_start = i;
        }
    }
    if temp_count > count {
        count = temp_count;
        start = temp_start;
    }
    Some(&s[start..(start + count)])
}
