use crate::rs_io::Scanner;

#[test]
fn sc_i32() {
    let mut sc = Scanner::new(&mut std::fs::File::open("./testcase/Scanner/i32test.in").ok().unwrap());
    let n = sc.read::<i32>();
    let v: Vec<i32> = (0..n).map(|_| sc.read::<i32>()).collect();
     
    assert_eq!(sc.empty(), true);
    assert_eq!(format!("{} {:?}", n, v),
                "7 [-182, 2701, 14843, 23105, 11912, -11086, 2166]");
}

#[test]
fn sc_str_to_chars() {
    let mut sc = Scanner::new(&mut std::fs::File::open("./testcase/Scanner/str.in").ok().unwrap());
    let v: Vec<char> = sc.read::<String>().chars().collect();
     
    assert_eq!(sc.empty(), true);
    assert_eq!(format!("{:?}", v),
               r#"['t', 'o', 'y', 'a', 'm', 'a', '1', '7', '1', '0', '?', ':', ';', '#', '.', '.', '#']"#);
}

#[test]
fn sc_vec_char() {
    let mut sc = Scanner::new(&mut std::fs::File::open("./testcase/Scanner/chars.in").ok().unwrap());
    let n: i32 = sc.read();
    let v: Vec<char> = (0..n).map(|_| sc.read::<char>()).collect();

    assert_ne!(sc.empty(), true);
    assert_eq!(format!("{} {:?}", n, v),
                "5 ['0', '9', 'z', '!', '?']");
}