use device_query::DeviceQuery;

fn scan_send(repeat: usize, scan: u16) {
 let last: usize;

 let mut previous: usize = 0;

 if repeat == 0 { last = 1; } else { last = repeat; }

 unsafe {
  let mut input_d = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };
  let mut input_u = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };

  *input_d.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: 0, dwFlags: winapi::um::winuser::KEYEVENTF_SCANCODE                                       , dwExtraInfo: 1, wScan: scan, time: 0 };
  *input_u.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: 0, dwFlags: winapi::um::winuser::KEYEVENTF_SCANCODE | winapi::um::winuser::KEYEVENTF_KEYUP, dwExtraInfo: 1, wScan: scan, time: 0 };

  while previous < last {
   winapi::um::winuser::SendInput(1, &mut input_d, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
   winapi::um::winuser::SendInput(1, &mut input_u, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);

   std::thread::sleep(std::time::Duration::from_millis(20));

   previous += 1;
  }//while previous < last {
 }//unsafe {
}//fn scan_send(repeat: usize, scan: u16) {

fn main() {
 let mut previous: Vec<device_query::Keycode> = vec![];

 let state: device_query::DeviceState = device_query::DeviceState::new();

 loop {
  let current: Vec<device_query::Keycode> = state.get_keys();

  if previous != current {
   println!("current: {:?}", current);

   previous = current.clone();

   if current.iter().position(|&key| key == device_query::Keycode::F     ).is_some() { scan_send( 2, 0x2A ); }
   if current.iter().position(|&key| key == device_query::Keycode::Grave ).is_some() { scan_send( 6, 0x2A ); }
   if current.iter().position(|&key| key == device_query::Keycode::LAlt  ).is_some() { scan_send( 2, 0x1D ); }
   if current.iter().position(|&key| key == device_query::Keycode::S     ).is_some() { scan_send( 4, 0x1D ); }
   if current.iter().position(|&key| key == device_query::Keycode::Tab   ).is_some() { scan_send( 6, 0x1D ); }
   if current.iter().position(|&key| key == device_query::Keycode::W     ).is_some() { scan_send( 4, 0x2A ); }

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
