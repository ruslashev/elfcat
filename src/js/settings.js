const settings = document.getElementById('settings');
const arrows = document.getElementById('arrows');
const arrow_input = document.getElementById('arrow_opacity_range');

function toggleVisibility(elem) {
    if (elem.style.display === "none" || elem.style.display === "") {
        elem.style.display = "block";
    } else {
        elem.style.display = "none";
    }
}

document.getElementById('settings_toggle').onclick = function() {
    toggleVisibility(settings);
}

function setArrowOpacity(e) {
    arrows.style.opacity = e.target.valueAsNumber / 100;
}

arrow_input.addEventListener('change', setArrowOpacity);
arrow_input.addEventListener('input', setArrowOpacity);
