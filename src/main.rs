use device_query::DeviceQuery;

fn scan_send(flag: u32, key: u16, repeat: u64, scan: u16) {
 let last: u64;
 let stop: u64;

 let mut previous: u64 = 0;

 if repeat == 0 { last = 1; } else { last = repeat; }

 stop = 200 / last;

 unsafe {
  let mut input_d = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };
  let mut input_u = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };

  *input_d.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: key, dwFlags: flag                                       , dwExtraInfo: winapi::um::winuser::GetMessageExtraInfo() as usize, wScan: scan, time: 0 };
  *input_u.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: key, dwFlags: flag | winapi::um::winuser::KEYEVENTF_KEYUP, dwExtraInfo: winapi::um::winuser::GetMessageExtraInfo() as usize, wScan: scan, time: 0 };

  while previous < last {
   std::thread::sleep(std::time::Duration::from_millis(stop)); println!("winapi::um::winuser::SendInput(d): {:?}", winapi::um::winuser::SendInput(1, &mut input_d, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32));
   std::thread::sleep(std::time::Duration::from_millis(stop)); println!("winapi::um::winuser::SendInput(u): {:?}", winapi::um::winuser::SendInput(1, &mut input_u, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32));

   previous += 1;
  }//while previous < last {
 }//unsafe {
}//fn scan_send(flag: u32, key: u16, repeat: usize, scan: u16) {

fn main() {
 let     keyboard: device_query::DeviceState  = device_query::DeviceState::new();
 let mut previous: Vec<device_query::Keycode> = vec![]                          ;

 loop {
  let current: Vec<device_query::Keycode> = keyboard.get_keys();

  if previous != current {
   previous = current.clone();

   println!("current: {:?}", current);

   if previous.len() > 0 {
    if current.iter().position(|&key| key == device_query::Keycode::Meta      ).is_some() { continue; } 

    if current.iter().position(|&key| key == device_query::Keycode::J         ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 02, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::Insert    ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 03, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::K         ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 04, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::Home      ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 05, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::L         ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 06, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::PageUp    ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 07, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::Semicolon ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 08, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::LShift    ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 10, 0x0036 ); }
    if current.iter().position(|&key| key == device_query::Keycode::Grave     ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_SCANCODE   , 0x00, 12, 0x0036 ); }

    if current.iter().position(|&key| key == device_query::Keycode::M         ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 02, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::Delete    ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 03, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::Comma     ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 04, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::End       ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 05, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::Dot       ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 06, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::PageDown  ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 07, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::Slash     ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 08, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::LControl  ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 10, 0xE01D ); }
    if current.iter().position(|&key| key == device_query::Keycode::Tab       ).is_some() { scan_send( winapi::um::winuser::KEYEVENTF_EXTENDEDKEY, 0xA3, 12, 0xE01D ); }

   }//if previous.len() > 0 {
  }//if previous != current {
 }//loop {
}//fn main() {
