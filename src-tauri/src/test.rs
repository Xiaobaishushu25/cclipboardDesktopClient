use std::collections::HashMap;

fn clone_vec(vec_ref: &Vec<i32>) -> Vec<i32> {
    // 克隆vec_ref以获得一个新的拥有所有权的Vec
    let cloned_vec = vec_ref.clone();
    cloned_vec
}
#[test]
fn sdsdsd() {
    // 创建一个原始的Vec
    let original_vec = vec![1, 2, 3, 4, 5];

    // 调用clone_vec函数来克隆Vec
    let cloned_vec = clone_vec(&original_vec);

    // cloned_vec现在是一个新的拥有所有权的Vec
    println!("{:?}", cloned_vec); // 输出: [1, 2, 3, 4, 5]
}
#[test]
fn remove_map() {
    let mut map = HashMap::new();
    map.insert("asd",1);
    map.insert("asd1",1);
    map.insert("as2d",1);
    map.insert("a2sd",1);
    map.insert("a3sd",1);
    if let Some(a) = map.get("asd"){
        if *a==1 {
            map.remove("asd");
        }
    };
    println!("{:?}",map);
}
#[test]
fn whoaim() {
    println!(
        "User's Name            whoami::realname():    {}",
        whoami::realname(),
    );
    println!(
        "User's Username        whoami::username():    {}",
        whoami::username(),
    );
    println!(
        "User's Language        whoami::lang():        {:?}",
        whoami::lang().collect::<Vec<String>>(),
    );
    println!(
        "Device's Pretty Name   whoami::devicename():  {}",
        whoami::devicename(),
    );
    println!(
        "Device's Hostname      whoami::hostname():    {}",
        whoami::hostname(),
    );
    println!(
        "Device's Platform      whoami::platform():    {}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():      {}",
        whoami::distro(),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env(): {}",
        whoami::desktop_env(),
    );
    println!(
        "Device's CPU Arch      whoami::arch():        {}",
        whoami::arch(),
    );
}