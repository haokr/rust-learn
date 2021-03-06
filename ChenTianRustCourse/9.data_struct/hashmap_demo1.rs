use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);

    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));
    
    map.remove(&'a');

    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    map.shrink_to_fit();
    explain("shrinked", &map);
}

fn explain2<K, V>(name: &str, map: &HashMap<K, V>) {
    println!(
        "{}: len: {}, cap: {}", name, map.len(), map.capacity()
    );
}

fn explain(name: &str, map: HashMap<K, V>) -> HashMap<K, V> { 
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) }; 
    println!( "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}", 
    name, arr[2], arr[3], arr[4], arr[5] ); 
    unsafe { std::mem::transmute(arr) }
}
