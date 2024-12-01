#[inline]
pub fn attempt_1(input: &str) -> i32 {
    let mut i = 0;
    let (left, right) =
        input
            .split(['\n', ' '])
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                (!num.is_empty()).then(|| {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(num.parse::<i32>().ok()?);
                    Some(i += 1)
                });
                (left, right)
            });

    left.into_iter().fold(0, |acc, left| {
        acc + left * right.iter().filter(|right| left == **right).count() as i32
    })
}

/// SLOWER!
#[inline]
pub fn optim_1(input: &str) -> i32 {
    let mut i = 0;
    let (left, right) =
        input
            .split_whitespace()
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                (!num.is_empty()).then(|| {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(num.parse::<i32>().ok()?);
                    Some(i += 1)
                });
                (left, right)
            });

    left.into_iter().fold(0, |acc, left| {
        acc + left * right.iter().filter(|right| left == **right).count() as i32
    })
}
