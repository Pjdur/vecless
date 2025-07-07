use vecless::List;

#[test]
fn test_new_list_is_empty() {
    let list: List<i32> = List::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_push_and_len() {
    let list = List::new().push(1).push(2).push(3);
    assert_eq!(list.len(), 3);
    assert_eq!(format!("{}", list), "[3, 2, 1]");
}

#[test]
fn test_add_preserves_order() {
    let list = List::new().add([1, 2, 3]);
    assert_eq!(format!("{}", list), "[1, 2, 3]");
}

#[test]
fn test_reverse() {
    let list = List::new().add([1, 2, 3]);
    let reversed = list.clone().reverse();
    assert_eq!(format!("{}", reversed), "[3, 2, 1]");
    assert_eq!(list.len(), reversed.len());
}

#[test]
fn test_append() {
    let a = List::new().add([1, 2]);
    let b = List::new().add([3, 4]);
    let combined = a.clone().append(b.clone());
    assert_eq!(format!("{}", combined), "[1, 2, 3, 4]");
    assert_eq!(combined.len(), a.len() + b.len());
}

#[test]
fn test_iter_sum() {
    let list = List::new().add([1, 2, 3]);
    let sum: i32 = list.iter().copied().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_into_iter_owned() {
    let list = List::new().add(["a", "b", "c"]);
    let collected: Vec<_> = list.clone().into_iter().collect();
    assert_eq!(collected, vec!["a", "b", "c"]);
}

#[test]
fn test_into_iter_borrowed() {
    let list = List::new().add(["x", "y"]);
    let mut collected = vec![];
    for item in &list {
        collected.push(*item);
    }
    assert_eq!(collected, vec!["x", "y"]);
}

#[test]
fn test_into_iter_mut() {
    let mut list = List::new().add(["a", "b"]);
    for item in &mut list {
        *item = match *item {
            "a" => "A",
            "b" => "B",
            _ => item,
        };
    }
    assert_eq!(format!("{}", list), "[A, B]");
}

#[test]
fn test_display_formatting() {
    let list = List::new().add([10, 20, 30]);
    assert_eq!(format!("{}", list), "[10, 20, 30]");
}
