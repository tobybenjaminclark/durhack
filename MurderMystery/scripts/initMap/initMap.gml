


function init_map(location) {
    if (global.client_socket != undefined) {
        var t_buffer = buffer_create(256, buffer_grow, 1);
        buffer_seek(t_buffer, buffer_seek_start, 0);

        var json_string = "{ \"INIT_MAP\": { \"loc_str\": \"" + string(location) + "\" } }";
        buffer_write(t_buffer, buffer_string, json_string);
        network_send_packet(global.client_socket, t_buffer, buffer_tell(t_buffer));
        buffer_delete(t_buffer);
    } else {
        show_message("No active TCP connection!");
    }
}


/// @function reciever_init_map(mapdata)
/// @desc Rebuilds a Map struct from server JSON and stores it in global.map
function reciever_init_map(mapdata) {
    // --- unwrap nesting (handles INIT_MAP.map etc) ---
    if (variable_struct_exists(mapdata, "INIT_MAP"))
        mapdata = mapdata.INIT_MAP;
    if (variable_struct_exists(mapdata, "map"))
        mapdata = mapdata.map;

    if (!is_struct(mapdata)) return;

    // --- extract fields safely ---
    if (!variable_struct_exists(mapdata, "locations")) return;
    if (!variable_struct_exists(mapdata, "routes")) return;

    var locs_raw   = mapdata.locations;
    var routes_raw = mapdata.routes;
    var name       = (variable_struct_exists(mapdata, "name")) ? mapdata.name : "Received Map";

    if (!is_array(locs_raw)) return;
    if (!is_array(routes_raw)) return;

    // --- rebuild locations as Location structs ---
    var locations = [];
    for (var i = 0; i < array_length(locs_raw); i++) {
        var l = locs_raw[i];
        var loc = new Location(l.name, l.coords[0], l.coords[1], l._type);
        array_push(locations, loc);
    }

    // --- rebuild routes (inline find by name) ---
    var routes = [];
    for (var i = 0; i < array_length(routes_raw); i++) {
        var pair = routes_raw[i];
        if (!is_array(pair) || array_length(pair) < 2) continue;

        var a_raw = pair[0];
        var b_raw = pair[1];
        var a = undefined;
        var b = undefined;

        // inline lookup for matching Location objects
        for (var j = 0; j < array_length(locations); j++) {
            if (locations[j].name == a_raw.name) a = locations[j];
            if (locations[j].name == b_raw.name) b = locations[j];
        }

        if (is_undefined(a) || is_undefined(b)) continue;
        array_push(routes, [a, b]);
    }

    // --- final map struct ---
    global.map = new Map(locations, routes, name);
}
