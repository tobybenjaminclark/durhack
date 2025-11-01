/// Create Event
grid_cols   = 3;
grid_rows   = 4;
slot_size   = 80;
slot_margin = 32;
text_margin = 4;
title_text  = "Characters";

// --- compute grid dimensions ---
var total = array_length(global.characters);
var rows_used = ceil(total / grid_cols);
window_width  = (slot_size * grid_cols) + (slot_margin * (grid_cols - 1)) + 40; // + padding
window_height = (rows_used * (slot_size + 32)) + (slot_margin * (rows_used - 1)) + 80;

// --- center window ---
window_x = (display_get_gui_width()  - window_width)  * 0.5;
window_y = (display_get_gui_height() - window_height) * 0.8;
