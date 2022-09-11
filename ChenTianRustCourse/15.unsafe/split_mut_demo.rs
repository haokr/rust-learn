fn main() {
    let mut s = "你好，上海".to_string();
    let r = s.as_mut();
    if let Some((s1, s2)) = split_mut(r, '，') {
        println!("s1: {}  s2: {}", s1, s2);
    }
}

fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);

    pos.map(move |pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();

        // 将 s 转为裸指针
        let ss = s.as_mut_ptr();

        unsafe {
            // 根据指针+长度转为 slice
            let s1 = std::slice::from_raw_parts_mut(ss, pos);
            let s2 = std::slice::from_raw_parts_mut(ss.add(pos + sep_len), len - pos - sep_len);
            (std::str::from_utf8_unchecked_mut(s1), std::str::from_utf8_unchecked_mut(s2))
        }
    })
}