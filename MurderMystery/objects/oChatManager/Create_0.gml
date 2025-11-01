/// Create Event
window_width  = 500;
line_height   = 22;     // line spacing between messages
num_visible   = 8;      // number of visible messages


input_text = "";
enabled_keys = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 1234567890.,!?'-\"";
max_chars  = 120;

blink = true;
blink_speed = 15;
alarm[0] = blink_speed;

// --- layout ---
padding = 20;
input_height = 30;

window_height = (padding * 3) + (line_height * num_visible) + input_height;

// centre box in GUI space
window_x = (display_get_gui_width()  - window_width)  * 0.5;
window_y = (display_get_gui_height() - window_height) * 0.8;

// button and input positioning (relative)
input_x = window_x + padding;
input_y = window_y + window_height - input_height - padding;

input_w = window_width - (padding * 3) - 100; // leave room for button
btn_w   = 80;
btn_h   = input_height;
btn_x   = input_x + input_w + padding;
btn_y   = input_y;

btn_hover_alpha = 0;
btn_hover_speed = 0.15; // how fast it fades (0.05 = slow, 0.3 = snappy)
