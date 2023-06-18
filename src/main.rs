use device_query::DeviceQuery;

fn scan_send(scan: u16) {
 unsafe {
  let mut input = winapi::um::winuser::INPUT { type_: winapi::um::winuser::INPUT_KEYBOARD, u: std::mem::zeroed() };

  *input.u.ki_mut() = winapi::um::winuser::KEYBDINPUT { wVk: 0, dwFlags: winapi::um::winuser::KEYEVENTF_SCANCODE, dwExtraInfo: 1, wScan: scan, time: 0 };

  winapi::um::winuser::SendInput(1, &mut input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);

  input.u.ki_mut().dwFlags = winapi::um::winuser::KEYEVENTF_KEYUP | winapi::um::winuser::KEYEVENTF_SCANCODE;

  winapi::um::winuser::SendInput(1, &mut input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
 }//unsafe {
}//fn scan_send(scan: u16) {

fn sleep(duration: u64) { std::thread::sleep(std::time::Duration::from_millis(duration)); }

fn main() {
 let     duration: u64                        = 20                              ;
 let mut previous: Vec<device_query::Keycode> = vec![]                          ;
 let     state   : device_query::DeviceState  = device_query::DeviceState::new();

 loop {
  let current: Vec<device_query::Keycode> = state.get_keys();

  if previous != current {
   println!("current: {:?}", current);

   previous = current.clone();

   if current.iter().position(|&key| key == device_query::Keycode::LAlt ).is_some() {
    scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D );

   }//if current.iter().position(|&key| key == device_query::Keycode::LAlt ).is_some() {

   if current.iter().position(|&key| key == device_query::Keycode::RAlt ).is_some() {
    scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A );

   }//if current.iter().position(|&key| key == device_query::Keycode::RAlt ).is_some() {

   if current.iter().position(|&key| key == device_query::Keycode::S ).is_some() {
    scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D ); sleep(duration); scan_send( 0x1D );

   }//if current.iter().position(|&key| key == device_query::Keycode::S ).is_some() {

   if current.iter().position(|&key| key == device_query::Keycode::W ).is_some() {
    scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A ); sleep(duration); scan_send( 0x2A );

   }//if current.iter().position(|&key| key == device_query::Keycode::W ).is_some() {

   if current.iter().position(|&key| key == device_query::Keycode::Tab).is_some() {
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
