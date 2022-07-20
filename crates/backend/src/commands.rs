#[tauri::command]
pub fn my_custom_command(state: tauri::State<crate::MyState>,state_two: tauri::State<crate::MyStateTwo>) {
  println!("I was invoked from JS! {} : {}", state.0, state_two.0);
}
