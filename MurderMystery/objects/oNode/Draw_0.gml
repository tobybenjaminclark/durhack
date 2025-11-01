/// oNode - Draw Event
if (!is_struct(loc_data)) exit;

var name = loc_data.name;
var _x = x;
var _y = y;

draw_set_color(c_white);
draw_circle(_x, _y, node_r, false);

draw_set_color(c_white);
draw_text(_x + node_r + 2, _y - string_height(name) * 0.5, name);
