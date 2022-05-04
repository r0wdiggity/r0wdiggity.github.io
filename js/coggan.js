import init, {calculate_power} from "../rust/pkg/rust.js";

export function get_power_data(weight, threshold, five_min, one_min, five_sec, gender){
    weight = parseFloat(document.forms[0].elements["weight"].value)
    threshold = parseFloat(document.forms[0].elements["threshold"].value)
    five_min = parseFloat(document.forms[0].elements["five_min"].value)
    one_min = parseFloat(document.forms[0].elements["one_min"].value)
    five_sec = parseFloat(document.forms[0].elements["five_sec"].value)
    gender = document.forms[0].elements["gender"].value == 'male'
    init()
        .then(() => {
            let power_numbers = (calculate_power(weight, threshold, five_min, one_min, five_sec, gender));
            //power_numbers = power_numbers.replace(/(\n)+/g, '<br />');
            document.getElementById("power_results").innerHTML = power_numbers
        });
    return false;
}

window.power_data = function power_data(){
    get_power_data()
}