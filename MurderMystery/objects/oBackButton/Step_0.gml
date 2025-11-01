// --- BACK BUTTON CLICK ---
var mx = device_mouse_x_to_gui(0);
var my = device_mouse_y_to_gui(0);
if (mouse_check_button_pressed(mb_left)) {
    if (mx > back_x && mx < back_x + back_w && my > back_y && my < back_y + back_h) {
        room_goto(back_target_room);
    }
}
