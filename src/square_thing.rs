fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    let long = if lng > wdth { lng } else { wdth };
    let short = if lng < wdth { lng } else { wdth };
    let biggest_square = short;
    let remaining_dimensions = (long - short, short);

    println!("{}{}", long, short);
    if long == short {
        return None;
    }

    if biggest_square == 1 {
        return Some(vec![1 as i32; long as usize]);
    }

    let smaller_squares = sq_in_rect(remaining_dimensions.0, remaining_dimensions.1);

    match smaller_squares {
        Some(smaller_squares) => Some([vec![biggest_square], smaller_squares].concat()),
        None => Some(vec![biggest_square]),
    }
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
