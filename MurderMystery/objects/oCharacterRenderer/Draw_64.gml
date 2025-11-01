/// Draw GUI Event
draw_set_font(-1);
draw_set_color(c_white);

var total = array_length(global.characters);
var cols = grid_cols;

// --- compute title position ---
var title_w = string_width(title_text);
var title_x = window_x + (window_width * 0.5) - (title_w * 0.5);
var title_y = window_y - 36; // moved *above* the box

// --- draw title ---
draw_set_color(c_white);
draw_text(title_x, title_y, title_text);

// --- draw background window ---
draw_set_color(make_color_rgb(25, 25, 25));
draw_rectangle(window_x, window_y, window_x + window_width, window_y + window_height, false);

// --- compute grid start position (with padding) ---
var grid_start_x = window_x + 20;
var grid_start_y = window_y + 20; // no need to leave space for title now

// --- draw character slots ---
for (var i = 0; i < total; i++) {
    var col = i mod cols;
    var row = i div cols;

    var _x = grid_start_x + col * (slot_size + slot_margin);
    var _y = grid_start_y + row * (slot_size + 32 + slot_margin);

    var name = string(global.characters[i]);

    // --- draw box ---
    draw_set_color(make_color_rgb(60, 60, 60));
    draw_rectangle(_x, _y, _x + slot_size, _y + slot_size, false);

    // --- draw name ---
    var text_w = string_width(name);
    var text_x = _x + (slot_size * 0.5) - (text_w * 0.5);
    var text_y = _y + slot_size + text_margin;

    draw_set_color(c_white);
    draw_text(text_x, text_y, name);
}
