/// Step Event



// --- can only type if last message is from AI ---
var can_type = false;
if (array_length(global.current_messages) > 0) {
    var last_msg = global.current_messages[array_length(global.current_messages) - 1];
    if (!last_msg.is_player) can_type = true;
} else {
    can_type = true; // allow typing if chat just started
}



// --- block input if not allowed ---
if (!can_type) exit;



// --- otherwise: handle single-character input ---
for (var k = 32; k <= 126; k++) {
    if (keyboard_check_pressed(k)) {
        var ch = chr(k);

        // handle shift for letters
        if (k >= ord("A") && k <= ord("Z")) {
            if (!keyboard_check(vk_shift)) {
                ch = string_lower(ch);
            }
        }

        // only add if allowed
        if (string_count(ch, enabled_keys)) {
            blink = true;
            alarm[0] = blink_speed;

            switch (ch) {
                case "#": input_text += "\#"; break;
                default:  input_text += ch; break;
            }

            // truncate
            if (string_length(input_text) > max_chars) {
                input_text = string_copy(input_text, 1, max_chars);
            }
        }
    }
}



// --- handle backspace ---
if (keyboard_check_pressed(vk_backspace)) {
    if (string_length(input_text) > 0) {
        input_text = string_copy(input_text, 1, string_length(input_text) - 1);
    }
}



// --- handle paste ---
if (keyboard_check(vk_control) && keyboard_check_pressed(ord("V"))) {
    input_text += clipboard_get_text();
    if (string_length(input_text) > max_chars) {
        input_text = string_copy(input_text, 1, max_chars);
    }
}



// --- handle send (Enter or click) ---
if (keyboard_check_pressed(vk_enter)) {
    send_message();
}



if (mouse_check_button_pressed(mb_left)) {
    var mx = device_mouse_x_to_gui(0);
    var my = device_mouse_y_to_gui(0);
    if (mx > btn_x && mx < btn_x + btn_w && my > btn_y && my < btn_y + btn_h) {
        send_message();
    }
}



// --- update hover alpha for send button ---
var mx = device_mouse_x_to_gui(0);
var my = device_mouse_y_to_gui(0);
var hovering = (mx > btn_x && mx < btn_x + btn_w && my > btn_y && my < btn_y + btn_h);

btn_hover_alpha = lerp(btn_hover_alpha, hovering ? 1 : 0, btn_hover_speed);