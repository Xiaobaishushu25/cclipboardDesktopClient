use rand::Rng;
use rand::thread_rng;


// fn random_string(length: usize) -> String {
//     let characters: Vec<char> = ('a'..'z').chain(('A'..'Z')).chain((0..10).map(|n| (n as char) + '0')).collect();
//     let rng = &mut rng();
//     let random_chars: Vec<char> = rng.gen_iter::<char, _>(&characters).take(length).collect();
//     random_chars.iter().collect::<String>()
// }
///生成随机9位id
pub fn generate_id() -> String {
    // 定义可能的字符集
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    // 创建一个随机数生成器
    let mut rng = thread_rng();
    // 生成随机字符串
    let random_string: String = (0..9)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    random_string
}
// fn generate_pair_number() -> u64 {
//     let mut rng = thread_rng();
//     let number: u64 = rng.gen_range(100000..1000000); // 生成100000到999999之间的随机数
//     number
// }
// #[test]
// fn testss() {
//     let mut count = 0;
//     let result = loop {
//         count += 1;
//
//         if count ==10{
//             break count *10;
//         }
//     };
//     println!("{}",result);
// }