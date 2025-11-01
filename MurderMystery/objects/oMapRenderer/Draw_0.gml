/// oMap - Draw Event

// --- draw background ---
draw_set_color(col_bg);
draw_rectangle(x0, y0, x0 + map_w, y0 + map_h, false);

// --- draw routes ---
if (global.map != undefined && is_struct(global.map)) {
    draw_set_color(col_route);
    for (var i = 0; i < array_length(global.map.routes); i++) {
        var route = global.map.routes[i];
        var a = route[0];
        var b = route[1];

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

        var col = col_node;
        if (j == hover_index) col = col_hover;

        // --- draw filled square ---
        draw_set_color(col);
        draw_rectangle(_x - node_half, _y - node_half, _x + node_half, _y + node_half, false);

        // --- draw selection outline ---
        if (global.current_location == loc.name) {
            draw_set_color(col_outline);
            draw_rectangle(_x - node_half - 2, _y - node_half - 2, _x + node_half + 2, _y + node_half + 2, false);
        }
    }
}
