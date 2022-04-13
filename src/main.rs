fn get_uniq_chars(_s: &String) -> Vec<String> {
    let mut uniq_chars: Vec<String> = Vec::new();
    let s = _s.to_ascii_lowercase();
    for _c in s.chars() {
        let c = String::from(_c);
        if !uniq_chars.contains(&c) {
            uniq_chars.push(c);
        }
    }
    return uniq_chars;
}

fn vector_with_zeros(n: i64) -> Vec<i64> {
    let mut zero_vector: Vec<i64> = Vec::new();
    for _i in 0..n {
        zero_vector.push(0);
    }
    return zero_vector;
}

fn string_distance(s1: &String, s2: &String) -> f64{
    if s1 == s2 {
        return 1.0
    }
    let len1: i64 = s1.len() as i64;
    let len2: i64 = s2.len() as i64;

    let max_dist: f64 = std::cmp::max(len1, len2) as f64/2.0;
    let max_dist: i64 = (max_dist - 1.0) as i64;

    let mut fmatch: i64 = 0;

    let mut hash_s1: Vec<i64> = vector_with_zeros(len1);
    let mut hash_s2: Vec<i64> = vector_with_zeros(len2);

    for i in 0..len1 {
        for j in std::cmp::max(0, i - max_dist)..std::cmp::min(len2, i + max_dist + 1) {
            if s1.get(i as usize..(i+1) as usize) == s2.get(j as usize..(j+1) as usize) && hash_s2[j as usize] == 0 {
                hash_s1[i as usize] = 1;
                hash_s2[j as usize] = 1;
                fmatch += 1;
                break;
            }
        }
    }

    if fmatch == 0 {
        return 0.0;
    }

    let mut t: f64 = 0.0;
    let mut point: i64 = 0;

    for i in 0..len1 {
        if hash_s1[i as usize] == 1 {
            while hash_s2[point as usize] == 0 {
                point += 1;
            }

            if s1.get(i as usize..(i+1) as usize) != s2.get(point as usize..(point+1) as usize) {
                t += 1.0
            }
            point += 1;
        } 
    }
    t = t / 2.0;

    return ((fmatch as f64) / (len1 as f64) + (fmatch as f64) / (len2 as f64) + (fmatch as f64 - t) / (fmatch as f64)) / 3.0;
}

fn get_uniq_chars_in_vec(b: &Vec<&str>) -> Vec<String> {
    let mut uniq_chars: Vec<String> = Vec::new();
    for _word in b {
        let word = _word.to_ascii_lowercase();
        for _c in word.chars() {
            let c = String::from(_c);
            if !uniq_chars.contains(&c) {
                uniq_chars.push(c);
            }
        }
    }
    return uniq_chars;
}

fn tofind_word_percentage(result: &Vec<f64>) -> f64 {
    let mut total: f64 = 0.0;
    let mut maximum: i64 = 0;
    for res in result {
        total += *res;
        maximum += 1;
    }
    return (total/maximum as f64)*100f64;
}

fn calculate_tofind_word(s: &str, tofindwords: &Vec<&str>) -> f64 {
    //let mut wordfound = String::new();
    let mut highestdistance: f64 = 0.0;
    for word in tofindwords {
        let distance: f64 = string_distance(&String::from(*word), &String::from(s));
        if distance > highestdistance {
            highestdistance = (string_distance(&String::from(*word), &String::from(s)) + distance)/2.0;
        }
    }
    return highestdistance;
}

fn detect(s: &String, b: &Vec<&str>) -> Vec<f64> {
    let mut results: Vec<f64> = vec![];
    for c in s.chars() {
        let spl: Vec<&str> = s.split(c).collect();
        for block in spl {
            results.push(calculate_tofind_word(block, b));
            let mut temp_block = String::from(block);
            for crwucr in get_uniq_chars(&String::from(block)) {
                for uctr in get_uniq_chars_in_vec(b) {
                    temp_block = temp_block.replace(&crwucr, &uctr);
                    results.push(calculate_tofind_word(&temp_block, b));
                }
            }
            for (i, _c) in block.chars().enumerate() {
                let mut temp_s = String::from(block.clone());
                temp_s.remove(i);
                //println!("{}", temp_s);
                results.push(calculate_tofind_word(&temp_s, b));
            }
        }
    }
    return results;
}

fn main() {
    let tofindwords = vec![
        "merhaba"
    ];
    let example = String::from("m3r haba");
    let results = &detect(&example, &tofindwords);
    //println!("{:?}", results);
    let results = tofind_word_percentage(results);
    println!("%{:?} detected", results);
}
