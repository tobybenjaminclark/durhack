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

node_r    = 6;
col_bg    = make_color_rgb(25, 25, 25);
col_route = make_color_rgb(120, 120, 120);
col_node  = c_white;
col_label = c_white;



/// @function spawn_nodes_from_map()
function spawn_nodes_from_map() {
    if (!variable_global_exists("map")) return;
    if (!is_struct(global.map)) return;
    if (!variable_struct_exists(global.map, "locations")) return;

    // delete existing node instances first
    with (oNode) instance_destroy();

    // spawn each node
    var locs = global.map.locations;
    for (var i = 0; i < array_length(locs); i++) {
        var loc = locs[i];
        if (!is_struct(loc)) continue;

        var _x = x0 + loc.coords[0] * map_w;
        var _y = y0 + loc.coords[1] * map_h;

        var inst = instance_create_layer(_x, _y, "Instances", oNode);
		inst.x = _x;
		inst.y = _y;
        inst.loc_data = loc; // pass full struct (name, coords, _type, etc.)
        inst.node_r   = node_r;
    }
}
