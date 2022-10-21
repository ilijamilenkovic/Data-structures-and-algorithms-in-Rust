use data_structures_and_algorithms_in_rust::linked_list::LinkedList;

fn main() {
    let mut ll = LinkedList::new();
    ll.push_head(3);
    ll.push_back(4);
    let back = ll.pop_back().unwrap();
    ll.push_in_position(0, 0).unwrap();
    ll.push_in_position(1, 1);
    ll.push_in_position(2, 2);
    ll.push_in_position(1, 123);

    let test = ll.pop_from_position(1).unwrap();

    println!("{}", test);
    ll.traverse();
    //println!("{}", ll);

    println!("{}", back);
}
