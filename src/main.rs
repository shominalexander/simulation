use device_query::DeviceQuery                 ;
use winapi::um::winuser::KEYEVENTF_EXTENDEDKEY;
use winapi::um::winuser::KEYEVENTF_SCANCODE   ;

fn scan_send(flag: u32, key: u16, repeat: usize, scan: u16) {
 let last: usize;

 let mut previous: usize = 0;

 if repeat == 0 { last = 1; } else { last = repeat; }

 unsafe {
  let mut input_d = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };
  let mut input_u = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };

  *input_d.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: key, dwFlags: flag                                       , dwExtraInfo: winapi::um::winuser::GetMessageExtraInfo() as usize, wScan: scan, time: 0 };
  *input_u.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: key, dwFlags: flag | winapi::um::winuser::KEYEVENTF_KEYUP, dwExtraInfo: winapi::um::winuser::GetMessageExtraInfo() as usize, wScan: scan, time: 0 };

  while previous < last {
   std::thread::sleep(std::time::Duration::from_millis(14));

   println!("winapi::um::winuser::SendInput(d): {:?}", winapi::um::winuser::SendInput(1, &mut input_d, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32));

   std::thread::sleep(std::time::Duration::from_millis(14));

   println!("winapi::um::winuser::SendInput(u): {:?}", winapi::um::winuser::SendInput(1, &mut input_u, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32));

   std::thread::sleep(std::time::Duration::from_millis(14));

   previous += 1;
  }//while previous < last {
 }//unsafe {
}//fn scan_send(flag: u32, key: u16, repeat: usize, scan: u16) {

fn main() {
 let mut previous: Vec<device_query::Keycode> = vec![];

 let state: device_query::DeviceState = device_query::DeviceState::new();

 loop {
  let current: Vec<device_query::Keycode> = state.get_keys();

  if previous != current {
   previous = current.clone();

   println!("current: {:?}", current);

   if current.iter().position(|&key| key == device_query::Keycode::Meta   ).is_some() {                                          continue; } 

   if current.iter().position(|&key| key == device_query::Keycode::G      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 02, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::B        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 02, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::F      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 04, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::V        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 04, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::D      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 06, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::C        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 06, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::S      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 08, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::X        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 08, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::A      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 10, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::Z        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 10, 0xE01D ); } 

   if current.iter().position(|&key| key == device_query::Keycode::E      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 03, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::R        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 03, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::Q      ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 05, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::W        ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 05, 0xE01D ); } 
   if current.iter().position(|&key| key == device_query::Keycode::Grave  ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 07, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::Tab      ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 07, 0xE01D ); } 

   if current.iter().position(|&key| key == device_query::Keycode::LShift ).is_some() { scan_send( KEYEVENTF_SCANCODE, 0x00, 12, 0x0036 ); } if current.iter().position(|&key| key == device_query::Keycode::LControl ).is_some() { scan_send( KEYEVENTF_EXTENDEDKEY, 0xA3, 12, 0xE01D ); } 

   if current.iter().position(|&key| key == device_query::Keycode::CapsLock).is_some() {
    winput::send_inputs( [ winput::Input::from_vk( winput::Vk::LeftWin, winput::Action::Press   )
                         , winput::Input::from_vk( winput::Vk::Tab    , winput::Action::Press   )
                         , winput::Input::from_vk( winput::Vk::Tab    , winput::Action::Release )
                         , winput::Input::from_vk( winput::Vk::LeftWin, winput::Action::Release )
                         ]
                       );
   }//if current.iter().position(|&key| key == device_query::Keycode::Tab).is_some() {
  }//if previous != current {
 }//loop {
}//fn main() {
