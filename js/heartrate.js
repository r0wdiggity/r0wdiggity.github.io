import init, {calculate_hr_zones} from "./rust.js";

export function get_hr_data(){
    let hr_max = document.forms[0].elements["hr_max"].value;
    let hr_rest = document.forms[0].elements["hr_rest"].value;
    init()
        .then(() => {
            let hr_numbers = (calculate_hr_zones(hr_max, hr_rest));
            document.getElementById("improvement_table").innerHTML = hr_numbers;
        });
    return false;
}

window.hr_data = function hr_data(){
    get_hr_data()
}