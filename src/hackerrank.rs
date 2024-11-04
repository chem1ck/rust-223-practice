#[test]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simpleArraySum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for val in ar{
        sum += val;
    }
    return sum;
}

fn test1() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0, 0];
    for i in 0..a.len(){
        if a[i] > b[i]{
            result[0] += 1;
        }
        if a[i] < b[i]{
            result[1] += 1;
        }
    }
    return result;
}

fn test2() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
#[test]
/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn aVeryBigSum(ar: &[i64]) -> i64 {
    let mut sum = 0;
    for val in ar{
        sum += val;
    }
    return sum;
}

fn test3() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = aVeryBigSum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 0..n{
        for _ in 0..(n - i - 1){
            print!(" ");
        }
        for _ in 0..(i + 1){
            print!("#");
        }
        println!();
    }
}

fn test4() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
#[test]
/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i64]) {
    let sum: i64 = arr.iter().sum();
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let min_sum = sum - max;
    let max_sum = sum - min;

    println!("{} {}", min_sum, max_sum);
}

fn test5() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
#[test]
/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut max = candles[0];
    let mut countMax = 0;
    for i in candles{
        if i > &max {
            max = *i;
        }
    }
    for j in candles{
        if *j == max {
            countMax += 1;
        }
    }
    return countMax;
}

fn test6() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
fn timeConversion(s: &str) -> String {
    // Извлекаем часы, минуты и секунды
    let hours: u32 = s[0..2].parse().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];
    let period = &s[8..]; // "AM" или "PM"

    // Преобразование часов в 24-часовой формат
    let hours = match period {
        "PM" if hours != 12 => hours + 12,
        "AM" if hours == 12 => 0,
        _ => hours,
    };

    // Форматируем строку в "HH:MM:SS"
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn test7() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade < 38 {
            grade
        } else {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        }
    }).collect()
}

fn test8() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
#[test]
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn test9() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .filter(|&&dist| a + dist >= s && a + dist <= t)
        .count();

    let orange_count = oranges.iter()
        .filter(|&&dist| b + dist >= s && b + dist <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn test10() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
#[test]
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        return if x1 == x2 { "YES".to_string() } else { "NO".to_string() };
    }
    if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
        return "YES".to_string();
    }
    "NO".to_string()
}

fn test11() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let gcd = |x: i32, y: i32| -> i32 {
        let mut a = x;
        let mut b = y;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };

    let lcm = |x: i32, y: i32| -> i32 {
        (x * y) / gcd(x, y)
    };

    let mut lcm_a = a[0];
    for &num in &a[1..] {
        lcm_a = lcm(lcm_a, num);
    }

    let mut gcd_b = b[0];
    for &num in &b[1..] {
        gcd_b = gcd(gcd_b, num);
    }

    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn test12() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
#[test]
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0]; // Return 0 for both records if the scores list is empty
    }

    let mut best_count = 0;
    let mut worst_count = 0;
    let mut best_score = scores[0];
    let mut worst_score = scores[0];

    for &score in &scores[1..] {
        if score > best_score {
            best_score = score;
            best_count += 1;
        } else if score < worst_score {
            worst_score = score;
            worst_count += 1;
        }
    }

    vec![best_count, worst_count]
}

fn test13() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}