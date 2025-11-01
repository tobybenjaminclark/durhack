/// oLocationDisplay - Draw GUI Event

if (!variable_global_exists("current_location")) exit;
if (global.current_location == "") exit; // nothing selected

var gui_w = display_get_gui_width();
var gui_h = display_get_gui_height();

var text = global.current_location;
var tw = string_width(text);
var th = string_height(text);

// --- position: centred in right-hand half of screen ---
var _x = gui_w * 0.75 - (tw * 0.5);
var _y = (gui_h * 0.5) - (th * 0.5);

// --- draw text ---
draw_set_color(col_text);
draw_text(_x, _y, text);
