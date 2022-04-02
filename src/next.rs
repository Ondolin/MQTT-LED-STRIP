match local_status {
0 => {
// off
if prev_status != local_status {
animation_status = vec![];
prev_status = local_status;
{
let mut strip_lock = strip.lock().unwrap();
strip_lock.reset();
}
}
},
2 => {
// rainbow fade
let initial_color: u32 = 0;
let step_size = 3;
if prev_status != local_status {
prev_status = local_status;
animation_status = vec![initial_color as i32];
let hsv_color = Hsv::new(Deg(animation_status[0] as f32), 1.0, 1.0);
let rgb_color = Rgb::from_color(&hsv_color);
let color = Color::from_rgb(rgb_color.red(), rgb_color.green(), rgb_color.blue());
{
let mut strip_lock = strip.lock().unwrap();
strip_lock.set_all(color);
}
}
animation_status[0] += step_size;
animation_status[0] %= 360;
let hsv_color = Hsv::new(Deg(animation_status[0] as f32), 1.0, 1.0);
let rgb_color = Rgb::from_color(&hsv_color);
let color = Color::from_rgb(rgb_color.red(), rgb_color.green(), rgb_color.blue());
{
let mut strip_lock = strip.lock().unwrap();
strip_lock.set_all(color);
}
},
3 => {
// full rainbow
if prev_status != local_status {

}
},
_ => {
// off (default)
if prev_status != 2 {
animation_status = vec![];
prev_status = 2;
{
let mut strip_lock = strip.lock().unwrap();
strip_lock.reset();
}
}
}
}