/// Draw Event
if (!variable_global_exists("map")) exit;
if (!is_struct(global.map)) exit;

// --- draw title above map ---
if (variable_struct_exists(global.map, "name")) {
    var title = global.map.name;
    var title_w = string_width(title);
    var title_x = x0 + (map_w * 0.5) - (title_w * 0.5);
    var title_y = y0 - 28;
    draw_set_color(c_white);
    draw_text(title_x, title_y, title);
}

// --- draw background rectangle ---
draw_set_color(col_bg);
draw_rectangle(x0, y0, x0 + map_w, y0 + map_h, false);

// helper to remap -1..1 → pixel coordinate inside map
function map_coord(_x, _y) {
    var px = x0 + (( _x + 1) * 0.5) * map_w;
    var py = y0 + ((-_y + 1) * 0.5) * map_h; // invert Y so +up visually
    return [px, py];
}

// --- draw routes ---
if (variable_struct_exists(global.map, "routes") && is_array(global.map.routes)) {
    draw_set_color(col_route);
    for (var i = 0; i < array_length(global.map.routes); i++) {
        var route = global.map.routes[i];
        var a, b;

        if (is_array(route)) {
            if (array_length(route) < 2) continue;
            a = route[0];
            b = route[1];
        } else if (is_struct(route)) {
            a = variable_struct_get(route, "0");
            b = variable_struct_get(route, "1");
        } else continue;

        if (!is_struct(a) || !is_struct(b)) continue;

        var pa = map_coord(a.coords[0], a.coords[1]);
        var pb = map_coord(b.coords[0], b.coords[1]);
        draw_line(pa[0], pa[1], pb[0], pb[1]);
    }
}

// --- draw nodes ---
if (variable_struct_exists(global.map, "locations") && is_array(global.map.locations)) {
    for (var j = 0; j < array_length(global.map.locations); j++) {
        var loc = global.map.locations[j];
        if (!is_struct(loc)) continue;

        var p = map_coord(loc.coords[0], loc.coords[1]);
        var _x = p[0];
        var _y = p[1];

        draw_set_color(col_node);
        draw_circle(_x, _y, node_r, false);

        draw_set_color(col_label);
        draw_text(_x + node_r + 2, _y - string_height(loc.name) * 0.5, loc.name);
    }
}
