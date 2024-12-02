#[inline]
pub fn attempt_2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        if !line.is_empty() {
            let mut nums = line.split(' ');
            let first = nums.next().unwrap().parse::<i8>().unwrap();
            let mut last = nums.next().unwrap().parse::<i8>().unwrap();
            let is_up = if last > first && last - 3 <= first {
                true
            } else if last < first && last + 3 >= first {
                false
            } else {
                return acc;
            };
            for n in nums {
                let this = n.parse::<i8>().unwrap();
                match is_up {
                    true if this > last && this - 3 <= last => last = this,
                    false if this < last && this + 3 >= last => last = this,
                    _ => return acc,
                }
            }
            acc + 1
        } else {
            acc
        }
    })
}

#[inline]
pub fn _attempt_1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        if !line.is_empty() {
            let mut nums = line.split(' ');
            let first = nums.next().unwrap().parse::<u8>().unwrap();
            let mut last = nums.next().unwrap().parse::<u8>().unwrap();
            let is_up = if last > first {
                true
            } else if last < first {
                false
            } else {
                return acc;
            };
            for n in nums {
                let this = n.parse::<u8>().unwrap();
                match is_up {
                    true if this > last => last = this,
                    false if this < last => last = this,
                    _ => return acc,
                }
            }

            acc + 1
        } else {
            acc
        }
    })
}
