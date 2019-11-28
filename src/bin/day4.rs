fn main() {
    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    let digest = md5::compute(b"");
    assert_eq!(format!("{:x}", digest), "d41d8cd98f00b204e9800998ecf8427e");
    let s = String::from("The quick brown fox jumps over the lazy dog");
    let digest = md5::compute(s);
    assert_eq!(format!("{:x}", digest), "9e107d9d372bb6826bd81d3542a419d6");

    let s = format!("abcdef{}", 609_043);
    let digest = md5::compute(s);
    let digest = format!("{:x}", digest);
    //assert_eq!( digest[0..5], (String::from("00000"))[0..5]);
    assert!(digest.starts_with("00000"));

    println!("Solution1: {}", get_solution("00000"));
    println!("Solution2: {}", get_solution("000000"));
}

fn get_solution(pattern: &str)->u32{
    let mut i:u32=0;
    loop{
        let digest = compute_md5(i);
        if digest.starts_with(pattern){
            break;
        }
        i+=1;
    }
    i
}

fn compute_md5(n: u32) -> String {
    let s = format!("iwrupvqb{}", n);
    let digest = md5::compute(s);
    format!("{:x}", digest)
}
