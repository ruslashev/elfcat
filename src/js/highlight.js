let primary_id_elem = document.getElementById("primary_id")
let secondary_id_elem = document.getElementById("secondary_id")
if (primary_id_elem !== null && secondary_id_elem !== null) {
    primary_id_elem.addEventListener("mouseenter", function(event) {
        event.target.style.backgroundColor = "color";
        secondary_id_elem.style.backgroundColor = "color";
    }, false);
    primary_id_elem.addEventListener("mouseleave", function(event) {
        event.target.style.backgroundColor = "";
        secondary_id_elem.style.backgroundColor = "";
    }, false);
    secondary_id_elem.addEventListener("mouseenter", function(event) {
        event.target.style.backgroundColor = "color";
        primary_id_elem.style.backgroundColor = "color";
    }, false);
    secondary_id_elem.addEventListener("mouseleave", function(event) {
        event.target.style.backgroundColor = "";
        primary_id_elem.style.backgroundColor = "";
    }, false);
}
