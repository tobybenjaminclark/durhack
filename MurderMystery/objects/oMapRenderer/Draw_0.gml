/// oMap - Draw Event
if (!variable_global_exists("map")) exit;
if (!is_struct(global.map)) exit;

// draw title
if (variable_struct_exists(global.map, "name")) {
    var title = global.map.name;
    var title_w = string_width(title);
    var title_x = x0 + (map_w * 0.5) - (title_w * 0.5);
    var title_y = y0 - 28;
    draw_set_color(c_white);
    draw_text(title_x, title_y, title);
}

// background
draw_set_color(col_bg);
draw_rectangle(x0, y0, x0 + map_w, y0 + map_h, false);

// routes
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

        var ax = x0 + a.coords[0] * map_w;
        var ay = y0 + a.coords[1] * map_h;
        var bx = x0 + b.coords[0] * map_w;
        var by = y0 + b.coords[1] * map_h;
        draw_line(ax, ay, bx, by);
    }
}
