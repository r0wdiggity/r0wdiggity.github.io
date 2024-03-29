import init, {calculate_power} from "./rust.js";

export function get_power_data(){
    let units = document.forms[0].elements["units"].value;
    let weight;
    if (units === "lbs") {
        weight = parseFloat(document.forms[0].elements["weight"].value) * 0.4536;
    } else { weight = parseFloat(document.forms[0].elements["weight"].value); }
    let threshold = parseFloat(document.forms[0].elements["threshold"].value)
    let five_min = parseFloat(document.forms[0].elements["five_min"].value)
    let one_min = parseFloat(document.forms[0].elements["one_min"].value)
    let five_sec = parseFloat(document.forms[0].elements["five_sec"].value)
    let gender = document.forms[0].elements["gender"].value == 'male'
    init()
        .then(() => {
            let power_numbers = (calculate_power(weight, threshold, five_min, one_min, five_sec, gender));
            console.log(power_numbers);
            let power_arr = power_numbers.split("&&");
            console.log(power_arr);
            document.getElementById("power_results").innerHTML = power_arr[0];
            document.getElementById("table_h").innerHTML = "Improvements Needed";
            document.getElementById("table_p").innerHTML = "Below is a table showing the wattage improvements needed to reach each levels. (Negative numbers indicate a power drop to that level)";
            document.getElementById("improvement_table").innerHTML = power_arr[1];
        });
    return false;
}

window.power_data = function power_data(){
    get_power_data()
}