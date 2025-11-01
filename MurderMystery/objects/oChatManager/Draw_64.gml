/// Draw GUI Event

// background box
draw_set_color(make_color_rgb(25, 25, 25));
draw_rectangle(window_x, window_y, window_x + window_width, window_y + window_height, false);

// --- draw chat character name (centered above box) ---
if (variable_global_exists("current_chat_character")) {
    var name = global.current_chat_character;
    var name_w = string_width(name);
    var name_x = window_x + (window_width * 0.5) - (name_w * 0.5);
    var name_y = window_y - 28; // position above chat box

    draw_set_color(c_white);
    draw_text(name_x, name_y, name);
}

// determine which messages to show (only last 4)
var total = array_length(global.current_messages);
var visible_count = 5;
var start = max(0, total - visible_count);
var y_pos = window_y + padding;

// draw message history
for (var i = start; i < total; i++) {
    var msg = global.current_messages[i];
    var msg_text = msg.content;
    var msg_width = string_width(msg_text);
    var msg_height = string_height(msg_text);

    // define bubble positions
    var bubble_padding = 8;
    var bubble_y1 = y_pos - 2;
    var bubble_y2 = y_pos + msg_height + 4;

    // AI message (left)
    if (!msg.is_player) {
        var bubble_x1 = window_x + padding;
        var bubble_x2 = bubble_x1 + msg_width + bubble_padding * 2;

        draw_set_color(make_color_rgb(35, 60, 90));
        draw_rectangle(bubble_x1, bubble_y1, bubble_x2, bubble_y2, false);

        draw_set_color(c_aqua);
        draw_text(bubble_x1 + bubble_padding, y_pos, msg_text);
    }
    // Player message (right)
    else {
        var bubble_x2 = window_x + window_width - padding;
        var bubble_x1 = bubble_x2 - msg_width - bubble_padding * 2;

        draw_set_color(make_color_rgb(60, 90, 60));
        draw_rectangle(bubble_x1, bubble_y1, bubble_x2, bubble_y2, false);

        draw_set_color(c_lime);
        draw_text(bubble_x2 - msg_width - bubble_padding, y_pos, msg_text);
    }

    // spacing between messages
    y_pos += msg_height + 5 + bubble_padding;
}

// --- check if player can type ---
var can_type = false;
if (total > 0) {
    var last_msg = global.current_messages[total - 1];
    if (!last_msg.is_player) can_type = true;
} else {
    can_type = true;
}


// --- draw input box ---
draw_set_color(c_white);
draw_rectangle(input_x, input_y, input_x + input_w, input_y + input_height, false);

if (can_type) {
    draw_set_color(c_black);
    draw_text(input_x + 8, input_y + 6, string_copy(input_text, 1, max_chars));

    // blinking cursor
    if (blink) {
        var tx = input_x + 8 + string_width(input_text);
        var ty = input_y + 6;
        draw_line(tx, ty, tx, ty + string_height("A"));
    }
} else {
    draw_set_color(make_color_rgb(120, 120, 120));
    draw_text(input_x + 8, input_y + 6, "One message at a time, please");
}

// --- send button (disabled if can't type) ---
var mx = device_mouse_x_to_gui(0);
var my = device_mouse_y_to_gui(0);
var is_hover = (mx > btn_x && mx < btn_x + btn_w && my > btn_y && my < btn_y + btn_h);

if (!can_type) {
    draw_set_color(make_color_rgb(60, 60, 60)); // dark gray (disabled)
} else if (is_hover) {
    draw_set_color(make_color_rgb(90, 90, 90)); // lighter gray when hovered
} else {
    draw_set_color(c_dkgray);                   // normal
}
draw_rectangle(btn_x, btn_y, btn_x + btn_w, btn_y + btn_h, false);

draw_set_color(can_type ? c_white : make_color_rgb(150, 150, 150));
draw_text(btn_x + 18, btn_y + 6, "Send");
