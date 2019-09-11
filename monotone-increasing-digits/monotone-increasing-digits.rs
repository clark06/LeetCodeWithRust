fn isIncreasing(n: i32) -> bool {
    let mut num = n;
    let mut lastDigits = num % 10;
    while(num > 9) {
        num = num / 10;
        let mut newDigits = num % 10;
        if (lastDigits < newDigits) {
            return false;
        }
        lastDigits = newDigits;
    }
    return true;
}
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut num = n + 1;
        let mut i = 1;
        while(!isIncreasing(num - 1)) {
            num = num - (num % (10 * i));
            i = i * 10;
        }
        return num - 1;
    }
}