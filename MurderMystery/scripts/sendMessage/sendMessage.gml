/// Script: send_message(content)
function send_message() {
    // ignore empty messages
    if (string_length(string_trim(input_text)) <= 0) return;

    // append player message to chat history
    var new_msg = MESSAGE(input_text, true);
    array_push(global.current_messages, new_msg);

    // clear input field
    input_text = "";

    // feedback (temporary)
    show_debug_message("Message sent!");
}
