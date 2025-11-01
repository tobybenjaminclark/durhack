// --- draw background rectangle ---
draw_set_color(col_bg);
draw_rectangle(x0, y0, x0 + map_w, y0 + map_h, false);

// --- draw routes ---
draw_set_color(col_route);
for (var i = 0; i < array_length(global.map.routes); i++) {
    var route = global.map.routes[i];
    var a = route[0];
    var b = route[1];

    // remap from [-1,1] to [0,1]
    var ax = x0 + ((a.coords[0] + 1) * 0.5) * map_w;
    var ay = y0 + ((a.coords[1] + 1) * 0.5) * map_h;
    var bx = x0 + ((b.coords[0] + 1) * 0.5) * map_w;
    var by = y0 + ((b.coords[1] + 1) * 0.5) * map_h;

    draw_line(ax, ay, bx, by);
}

// --- draw nodes ---
for (var j = 0; j < array_length(global.map.locations); j++) {
    var loc = global.map.locations[j];
    var _x = x0 + ((loc.coords[0] + 1) * 0.5) * map_w;
    var _y = y0 + ((loc.coords[1] + 1) * 0.5) * map_h;

    // draw node circle
    draw_set_color(col_node);
    draw_circle(_x, _y, node_r, false);

    // draw label
    draw_set_color(col_label);
    draw_text(_x + node_r + 2, _y - string_height(loc.name) * 0.5, loc.name);
}
