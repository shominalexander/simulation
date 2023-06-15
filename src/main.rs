use device_query::{ DeviceQuery, DeviceState };
use enigo::KeyboardControllable               ;
use simulate                                  ;
use std::mem                                  ;
use tfc::{ traits::KeyboardContext           };
use winapi::um::winuser::*                    ;
use winput::Vk                                ;

fn send_input_key(virtual_key: i32, down: bool) {
 unsafe {
  let mut input = INPUT { type_: INPUT_KEYBOARD, u: std::mem::zeroed() };

  *input.u.ki_mut() = KEYBDINPUT { wVk: virtual_key as u16, dwFlags: if down { 0 } else { KEYEVENTF_KEYUP }, dwExtraInfo: 1, wScan: 0, time: 0 };

  SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
 }//unsafe {
}//fn send_input_key(virtual_key: i32, up: bool) {

fn send_input_key_down(virtual_key: i32) { send_input_key(virtual_key, true ); }
fn send_input_key_up  (virtual_key: i32) { send_input_key(virtual_key, false); }

fn main() {
 let mut context : tfc::Context               = tfc::Context::new().unwrap();
 let mut enigo   : enigo::Enigo               = enigo::Enigo::new()         ;
 let mut previous: Vec<device_query::Keycode> = vec![]                      ;
 let     state   : DeviceState                = DeviceState::new()          ;

 loop {
  let current: Vec<device_query::Keycode> = state.get_keys();

  if previous != current {
   println!("current: {:?}", current);

   previous = current.clone();

   let size: usize = current.len();

   match size {
    1 => {
     if current[0] == device_query::Keycode::Escape { break; }

     if current[0] == device_query::Keycode::C { let _ = context.key_down(             tfc::Key::Meta ); let _ =    context.key_up(             tfc::Key::Meta ); }
     if current[0] == device_query::Keycode::E {           enigo.key_down(           enigo::Key::Meta );              enigo.key_up(           enigo::Key::Meta ); }
     if current[0] == device_query::Keycode::I {      send_input_key_down(                       0x5B );         send_input_key_up(                       0x5B ); }
     if current[0] == device_query::Keycode::S { let _ =  simulate::press( simulate::Key::LeftWindows ); let _ = simulate::release( simulate::Key::LeftWindows ); }
     if current[0] == device_query::Keycode::W {            winput::press(                Vk::LeftWin );           winput::release(                Vk::LeftWin ); }
    }//1 => {

    _ => { }
   }//match size {
  }//if previous != current {
 }//loop {
}//fn main() {
