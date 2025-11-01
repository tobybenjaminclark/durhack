/// oMap - Step Event
hover_index = -1;

if (global.map != undefined && is_struct(global.map)) {
    var mx = device_mouse_x_to_gui(0);
    var my = device_mouse_y_to_gui(0);

    for (var i = 0; i < array_length(global.map.locations); i++) {
        var loc = global.map.locations[i];
        var _x = x0 + ((loc.coords[0] + 1) * 0.5) * map_w;
        var _y = y0 + ((loc.coords[1] + 1) * 0.5) * map_h;

        if (point_in_rectangle(mx, my, _x - node_half, _y - node_half, _x + node_half, _y + node_half)) {
            hover_index = i;

            // --- click detection ---
            if (mouse_check_button_pressed(mb_left)) {
                global.current_location = loc.name;
            }
        }
    }
}
