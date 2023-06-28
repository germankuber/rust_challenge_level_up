fn median(mut a: Vec<f32>) -> Option<f32> {
    // Function that calculates the median of a given vector of f32 values.
    // The vector `a` is passed as a mutable reference.

    if a.is_empty() {
        // If the vector is empty, there are no elements to calculate the median.
        // Return None to indicate that the result is undefined.
        return None;
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    // Sort the elements in the vector in ascending order.
    // The `sort_by` method is used along with a closure that compares two elements (`x` and `y`)
    // using `partial_cmp` to handle cases where the elements are NaN (Not-a-Number).

    let n_elements_to_check = a.len();
    // Store the number of elements in the vector.

    let middle_element_position = n_elements_to_check / 2;
    // Calculate the index position of the middle element.

    let a_is_even = n_elements_to_check % 2 == 0;
    // Check if the number of elements is even.

    let med_return_value = if a_is_even {
        // If the number of elements is even, calculate the median by taking the average of the two middle elements.
        (a[middle_element_position] + a[middle_element_position - 1]) / 2.0
    } else {
        // If the number of elements is odd, the median is the middle element.
        a[middle_element_position]
    };

    Some(med_return_value)
    // Return the calculated median value wrapped in an Option as Some.
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
