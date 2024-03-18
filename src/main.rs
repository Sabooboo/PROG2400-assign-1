mod list;

fn main() {
    let mut my_list: list::List<i32> = list::new();
    // Append
    my_list.append(2);
    my_list.append(3);
    my_list.append(4);
    my_list.append(5);

    // Prepend
    my_list.prepend(1);

    // Insert at any index
    my_list.insert_at(2, 3);

    assert!(my_list.at(2) == Some(&3));

    // And remove from any index
    my_list.remove_at(2);

    // At
    assert!(my_list.at(0) == Some(&1));
    assert!(my_list.at(4) == Some(&5));

    let mut current = my_list.at(0);
    for i in 1..my_list.size() {
        let next = my_list.at(i);
        // verifying that the list is in the order it was made
        assert!(current < next);
        current = next;
    }

    // Size
    assert!(my_list.size() == 5);

    // Head
    assert!(my_list.head() == Some(&1));

    // Tail
    assert!(my_list.tail() == Some(&5));

    // Pop (moves the last element from the list and returns it)
    assert!(my_list.pop() == Some(5));

    // Contains
    assert!(my_list.contains(&4) == true);

    // Find (4 is at index 3)
    assert!(my_list.find(&4) == Some(3));
    assert!(my_list.find(&5) == None);
    
    println!("list: {}", my_list);

    /*
     * Given a list, split it into two sub lists
     * — one for the front half, and one for the
     * back half. If the number of elements is odd,
     * the extra element should go in the front list.
     * So FrontBackSplit() on the list {2, 3, 5, 7, 11}
     * should yield the two lists {2, 3, 5} and {7, 11}.
     */
    let (front, back) = list::from_vec(vec![2, 3, 5, 7, 11]).front_back_split();
    println!("front: {}", front);
    println!("back: {}", back);

    /*
     * Given two one-way-link lists
     * A and B (sorted or unsorted).
     * Sort them independently and
     * then Merge as list.
    */
    let list_a = list::from_vec(vec![9, 3, 1, 5, 7]);
    let list_b = list::from_vec(vec![2, 6, 4, 10, 8]);
    let merged_sorted = {
        let mut merged_sorted: list::List<i32> = list::new();
        let mut sorted_a = list_a.into_iter().collect::<Vec<i32>>();
        sorted_a.sort();
        let mut sorted_b = list_b.into_iter().collect::<Vec<i32>>();
        sorted_b.sort();
        let mut a_iter = sorted_a.into_iter();
        let mut b_iter = sorted_b.into_iter();
        let mut a = a_iter.next();
        let mut b = b_iter.next();
        while a.is_some() || b.is_some() {
            match (a, b) {
                (Some(a_val), Some(b_val)) => {
                    if a_val <= b_val {
                        merged_sorted.append(a_val);
                        a = a_iter.next();
                    } else {
                        merged_sorted.append(b_val);
                        b = b_iter.next();
                    }
                }
                (Some(a_val), None) => {
                    merged_sorted.append(a_val);
                    a = a_iter.next();
                }
                (None, Some(b_val)) => {
                    merged_sorted.append(b_val);
                    b = b_iter.next();
                }
                (None, None) => break,
            }
        }
        merged_sorted
    };
    println!("merged_sorted: {}", merged_sorted);

}
