


/// @function Location(name, coords_x, coords_y, _type)
/// @param name     string
/// @param coords_x real
/// @param coords_y real
/// @param _type    string
/// @returns struct representing a Location
function Location(name, coords_x, coords_y, _type) constructor {
    self.name    = name;
    self.coords  = [coords_x, coords_y];
    self._type   = _type;
}


