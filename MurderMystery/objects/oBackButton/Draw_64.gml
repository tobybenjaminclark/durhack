// --- BACK BUTTON ---
var mx = device_mouse_x_to_gui(0);
var my = device_mouse_y_to_gui(0);
back_hover = (mx > back_x && mx < back_x + back_w && my > back_y && my < back_y + back_h);

// background
if (back_hover) {
    draw_set_color(make_color_rgb(80, 80, 80));  // lighter when hovered
} else {
    draw_set_color(make_color_rgb(50, 50, 50));  // normal dark grey
}
draw_rectangle(back_x, back_y, back_x + back_w, back_y + back_h, false);

// label
draw_set_color(c_white);
var text_w = string_width(back_text);
var text_h = string_height(back_text);
draw_text(back_x + (back_w - text_w) * 0.5, back_y + (back_h - text_h) * 0.5, back_text);
