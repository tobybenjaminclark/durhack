


/// @function Map(locations, routes)
/// @param locations array of Location structs
/// @param routes    array of [Location, Location] pairs
/// @returns struct representing a Map
function Map(locations, routes) constructor {
    self.locations = locations;
    self.routes    = routes;
}



function init_fake_map() {
	// --- create locations (coords between 0 and 1) ---
	var loc_restaurant = new Location("The Oak Inn", 0.15, 0.35, "Restaurant");
	var loc_hotel      = new Location("Riverbank Hotel", 0.55, 0.60, "Hotel");
	var loc_school     = new Location("Trent Primary", 0.80, 0.20, "School");
	var loc_gym        = new Location("Iron Temple", 0.35, 0.75, "Gym");
	var loc_church     = new Location("St. George’s", 0.70, 0.45, "Church");
	var loc_bank       = new Location("Midlands Bank", 0.25, 0.10, "Bank");

	// --- group them into an array ---
	var all_locations = [
	    loc_restaurant,
	    loc_hotel,
	    loc_school,
	    loc_gym,
	    loc_church,
	    loc_bank
	];

	// --- define routes between them (as pairs of Location structs) ---
	var all_routes = [
	    [loc_restaurant, loc_hotel],
	    [loc_hotel, loc_church],
	    [loc_church, loc_school],
	    [loc_restaurant, loc_bank],
	    [loc_gym, loc_hotel]
	];

	// --- create global map object ---
	return new Map(all_locations, all_routes);
}