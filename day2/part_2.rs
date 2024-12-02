#[inline]
pub fn attempt_2(input: &str) -> i32 {
    fn line_is_correct<'i, I: Iterator<Item = (usize, &'i i8)>>(mut nums: I) -> Result<(), usize> {
        let mut last = nums.next().unwrap();
        let mut is_up = None;

        for this in nums {
            let diff = *this.1 - *last.1;
            match is_up {
                Some(true) if diff > 0 && diff <= 3 => {}
                Some(false) if diff < 0 && diff >= -3 => {}
                None if diff > 0 && diff <= 3 => {
                    is_up = Some(true);
                }
                None if diff < 0 && diff >= -3 => {
                    is_up = Some(false);
                }
                _ => return Err(this.0),
            }

            last = this
        }

        Ok(())
    }

    input.lines().fold(0, |acc, line| {
        if !line.is_empty() {
            let nums: Vec<i8> = line.split(' ').map(|n| n.parse::<i8>().unwrap()).collect();
            match line_is_correct(nums.iter().enumerate()) {
                Ok(_) => {
                    return acc + 1;
                }
                Err(i) => {
                    for i in 0..i + 1 {
                        if line_is_correct(nums.iter().enumerate().filter(|(ii, _)| *ii != i))
                            .is_ok()
                        {
                            return acc + 1;
                        }
                    }
                    acc
                }
            }
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
            let mut last = nums.next().unwrap().parse::<i8>().unwrap();
            let mut is_up = None;
            let mut is_dampened = false;

            for n in nums {
                let this = n.parse::<i8>().unwrap();
                match is_up {
                    Some(true) if this > last && this - 3 <= last => last = this,
                    Some(false) if this < last && this + 3 >= last => last = this,
                    None if this > last && this - 3 <= last => {
                        last = this;
                        is_up = Some(true);
                    }
                    None if this < last && this + 3 >= last => {
                        last = this;
                        is_up = Some(false);
                    }
                    _ if !is_dampened => {
                        is_dampened = true;
                    }
                    _ => return acc,
                }
            }

            println!("{line}");
            acc + 1
        } else {
            acc
        }
    })
}
