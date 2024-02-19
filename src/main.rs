mod list;

fn main() {
    let mut list: list::List<i32> = list::new();
    // Append
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);

    // Prepend
    list.prepend(1);

    // At
    assert!(list.at(0) == Some(&1));
    assert!(list.at(4) == Some(&5));

    let mut current = list.at(0);
    for i in 1..list.size() {
        let next = list.at(i);
        // verifying that the list is in the order it was made
        assert!(current < next);
        current = next;
    }

    // Size
    assert!(list.size() == 5);

    // Head
    assert!(list.head() == Some(&1));

    // Tail
    assert!(list.tail() == Some(&5));

    // Pop (moves the last element from the list and returns it)
    assert!(list.pop() == Some(5));

    // Contains
    assert!(list.contains(&4) == true);

    // Find (4 is at index 3)
    assert!(list.find(&4) == Some(3));
    assert!(list.find(&5) == None);
    
    println!("list: {}", list);
}
