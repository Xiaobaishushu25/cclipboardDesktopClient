interface DeviceInfo {
    socketAddr: string; // 对应Rust中的SocketAddr，这里假设它是一个字符串表示的地址
    deviceName: string;
    deviceType: string; // 假设DeviceType是一个已经定义的枚举或类型
}
