/// oMap - Create Event
var gui_w = display_get_gui_width();
var gui_h = display_get_gui_height();

var pad = 20;

map_w = (gui_w * 0.5) - (pad * 2);
map_h = map_w;

if (map_h > gui_h - (pad * 2)) {
    map_h = gui_h - (pad * 2);
    map_w = map_h;
}

x0 = pad; 
y0 = (gui_h * 0.5) - (map_h * 0.5);

// --- colours and sizes ---
node_size  = 32;
node_half  = node_size * 0.5;

col_bg     = make_color_rgb(25, 25, 25);
col_route  = make_color_rgb(100, 100, 100);
col_node   = make_color_rgb(200, 200, 200);
col_hover  = make_color_rgb(255, 255, 180);
col_label  = c_white;
col_outline = c_white;

hover_index  = -1;
global.current_location = "";
