// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_errors;
mod clipboard;
mod handler;
mod message;
mod test;
mod utils;

use std::net::{ SocketAddr};
use std::str::FromStr;
use crate::clipboard::clipboard::ClipboardMonitor;
use crate::handler::handler::Context;
use crate::message::message::Message::{ClipboardMessage, CloseMessage, DeviceChangeResponseMessage, NoPairDeviceResponseMessage, PairCodeResponseMessage, PairCreateMessage, PairDeviceInfosResponseMessage, PairRequestMessage, RemovePairRequestMessage, RemovePairResponseMessage, ServerReadyResponseMessage, WorkErrorMessage};
use crate::message::message::{DeviceInfo, Message};
use std::sync::{Mutex, OnceLock};
use std::thread;
use tauri::{Manager, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem,  WindowEvent, State};
use tauri_plugin_autostart::MacosLauncher;
use tokio::io::{ AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};

pub static DEVICE_INFO: OnceLock<DeviceInfo> = OnceLock::new();
pub type Btx = broadcast::Sender<Message>;
pub type Mtx = mpsc::Sender<Message>;
///状态，Mtx是用于tauri向handler发送消息，Btx会被复制一份给handler结构，然后订阅其rx在tauri接收handler发来的消息
/// bool是是否关闭到托盘
pub struct MyState(Mtx, Btx, Mutex<bool>);
#[tokio::main]
async fn main() {
    let (tx, ui_rx) = mpsc::channel::<Message>(32);
    let (ui_tx, _) = broadcast::channel::<Message>(32);
    let new_ui_tx = ui_tx.clone();
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("无法读取输入");
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let item1 = CustomMenuItem::new("1", "取消配对");
    let item2 = CustomMenuItem::new("2", "退出");
    let tray_menu = tray_menu.add_item(item1).add_item(item2);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("cclipboard");
    tauri::Builder::default()
        .manage(MyState(tx, ui_tx, Mutex::new(true)))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"]) /* arbitrary number of args to pass to your app */))
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    let state:State<MyState> = event.window().state();
                    // let sender = (&state.0).clone();
                    let sender = state.0.clone();
                    let mutex_guard = state.2.lock().unwrap();
                    if *mutex_guard {
                        println!("阻止默认关闭");
                        event.window().hide().unwrap();
                        api.prevent_close();
                    }else {
                        println!("请关闭窗口！");
                        tokio::spawn(async move {
                            sender
                                .send(CloseMessage())
                                .await
                                .unwrap();
                        });
                        event.window().close().unwrap();
                    }
                }
                WindowEvent::Destroyed => {}
                _ => {}
            }
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app,event| {
            match event {
                SystemTrayEvent::MenuItemClick { id,.. } => {
                    match id.as_str() {
                        "1" => {//取消配对
                            if let Some(info) = DEVICE_INFO.get(){
                                let handle = app.clone();
                                tokio::spawn(async move{
                                    let state = handle.state();
                                    remove_request(info.get_addr().to_string(), state).await.unwrap();
                                });
                            }
                        }
                        "2" => { //退出
                            if DEVICE_INFO.get().is_some(){
                                let handle = app.clone();
                                tokio::spawn(async move {
                                    let state: State<MyState> = handle.state();
                                    state
                                        .0
                                        .send(CloseMessage())
                                        .await
                                        .unwrap();
                                });
                            };
                            app.exit(0);
                        }
                        _ => {}
                    }
                }
                SystemTrayEvent::LeftClick { .. } => {
                    let main_window = app.get_window("main").unwrap();
                    if !main_window.is_visible().unwrap(){
                        main_window.show().unwrap();
                    }else if main_window.is_minimized().unwrap() {
                        main_window.unminimize().unwrap();
                    }
                    main_window.set_focus().unwrap();
                }
                SystemTrayEvent::RightClick {
                    position: p,
                    size: _,
                    ..
                } => {}
                SystemTrayEvent::DoubleClick { .. } => {}
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            pair_create,
            pair_request,
            get_self_info,
            remove_request,
            exit_setting,
            start_listen
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            // set_window_shadow(&main_window);
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                let addr = "127.0.0.1:8888";
                // let addr = "101.132.113.152:8888";
                // 连接到服务器
                match TcpStream::connect(addr).await {
                    Ok(stream) => {
                        let (tx, clip_rx) = mpsc::channel::<String>(64);
                        //should not spawn main loop of clipboard master on tokio runtime
                        // Please create dedicated thread to run it.
                        // Clipboard functionality is fully synchronous so it is impossible to have it integrated in tokio runtime as it is.
                        thread::spawn(move || {
                            let handler = ClipboardMonitor::new(tx);
                            clipboard_master::Master::new(handler).run().unwrap();
                        });
                        // initialize your app here instead of sleeping :)
                        // After it's done, close the splashscreen and display the main window
                        main_window.show().unwrap();
                        tokio::spawn(async move {
                            let mut context = Context::new(stream);
                            context.start_work(new_ui_tx, ui_rx, clip_rx).await;
                        });
                    }
                    Err(e) => {
                        main_window.show().unwrap();
                        //服务器没开
                        //called `Result::unwrap()` on an `Err` value: Os { code: 10061, kind: ConnectionRefused, message: "由于目标计算机积极拒绝，无法连接。" }
                        println!("{}", e)
                    }
                };
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn exit_setting<'r>(can_tray:bool, state:State<'r, MyState>) -> Result<(), ()> {
    let mut x = state.2.lock().unwrap();
    println!("进来设置要不要托盘{}",can_tray);
    // let x = &mut state.2;
    *x = can_tray;
    Ok(())
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//https://tauri.app/v1/guides/features/command/
#[tauri::command]
async fn pair_request<'r>(code: String, state: State<'r, MyState>) -> Result<(), ()> {
    state
        .0
        .send(PairRequestMessage(code, DEVICE_INFO.get().unwrap().clone()))
        .await
        .unwrap();
    Ok(())
}
#[tauri::command]
async fn pair_create<'r>(state: State<'r, MyState>) -> Result<(), ()> {
    state
        .0
        .send(PairCreateMessage(DEVICE_INFO.get().unwrap().clone()))
        .await
        .unwrap();
    Ok(())
}
#[tauri::command]
fn get_self_info() -> Option<DeviceInfo> {
    // let option = DEVICE_INFO.get();
    // match option{
    //     None => None,
    //     Some(info) => {
    //         Some(info.clone())
    //     }
    // }
    DEVICE_INFO.get().cloned()
}
#[tauri::command]
async fn remove_request<'r>(addr: String, state:State<'r,MyState>) -> Result<(),()> {
    if let Ok(socket_addr) = SocketAddr::from_str(&addr){
        state
            .0
            .send(RemovePairRequestMessage(socket_addr))
            .await
            .unwrap();
    }
    Ok(())
}
#[tauri::command]
async fn start_listen<'r>(
    state: State<'r, MyState>,
    app_handle: tauri::AppHandle,
) -> Result<(), ()> {
    let mut rx = state.1.subscribe();
    loop {
        tokio::select! {
            message = rx.recv() => {
                match message{
                    Ok(msg) => {
                        match msg{
                            ServerReadyResponseMessage(_) => {
                                // let info = DEVICE_INFO.get().unwrap().clone();
                                // println!(" app_handle.emit_all(self_info)");
                                // app_handle.emit_all("self_info",info).unwrap();
                            }
                            PairDeviceInfosResponseMessage(mut devices) => {
                                let self_info = DEVICE_INFO.get().unwrap();
                                devices.retain(|x| x!=self_info );
                                app_handle.emit_all("pair_devices",devices).unwrap();
                            }
                            PairCodeResponseMessage(code) => {
                                println!(" app_handle.emit_all(pair_code)");
                                app_handle.emit_all("pair_code",code).unwrap();
                            }
                            ClipboardMessage(_) => {}
                            NoPairDeviceResponseMessage() => {
                                println!(" app_handle.emit_all(no_pair_device)");
                                app_handle.emit_all("no_pair_device","").unwrap();
                            }
                            DeviceChangeResponseMessage(b,info) => {
                                if b{
                                    println!("添加设备{:?}",info);
                                    app_handle.emit_all("add_device",info).unwrap();
                                }else{
                                    println!("移除设备{:?}",info);
                                    app_handle.emit_all("remove_device",info).unwrap();
                                }
                            }
                            RemovePairRequestMessage(_) => {}
                            RemovePairResponseMessage() => {
                                app_handle.emit_all("remove_self","").unwrap();
                            }
                            WorkErrorMessage() =>{
                                println!("服务器错误");
                                app_handle.emit_all("server_error","").unwrap();
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {println!("rx.recv()出现错误:{}",e)}
                }
            }
        }
    }
}
