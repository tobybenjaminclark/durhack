var col = global.is_connected ? c_lime : c_red;

// Draw shadow/background
draw_set_color(c_black);
draw_circle(room_width - 5 + 1, room_height - 5 + 1, radius, false);

// Draw indicator
draw_set_color(col);
draw_circle(room_width - 5, room_height - 5, radius, false);