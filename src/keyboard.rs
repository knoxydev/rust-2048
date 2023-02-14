pub mod keyboard_md
{
  use crate::Main;
	use device_query::{DeviceQuery, DeviceState, Keycode};
	//use std::any::type_of;


	pub fn start(main: &Main)
	{
		let device_state = DeviceState::new();
    let mut prev_keys = vec![];

    loop
    {
      let keys = device_state.get_keys();
      if keys != prev_keys && !keys.is_empty()
      {
        if keys.contains(&Keycode::Up) { main.up(); }
        if keys.contains(&Keycode::Left) { main.left(); }
        if keys.contains(&Keycode::Down) { main.down(); }
        if keys.contains(&Keycode::Right) { main.right(); }
      }
      prev_keys = keys;
    }

	}
}

