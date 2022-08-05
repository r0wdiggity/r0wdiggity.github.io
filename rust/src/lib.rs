use wasm_bindgen::prelude::*;

mod coggan;
mod heartrate;

use coggan::*;
use heartrate::HRZones;

#[wasm_bindgen]
pub fn calculate_power(weight: f32, threshold: f32, five_min: f32, one_min: f32, five_sec: f32, gender: bool) -> String {
    let curves: coggan::Curves = match gender {
        true => coggan::Curves {threshold: coggan::constants::MALE_THRESHOLD, five_min: coggan::constants::MALE_FIVE_MIN, one_min: coggan::constants::MALE_ONE_MIN, five_sec: coggan::constants::MALE_FIVE_SEC },
        false =>  coggan::Curves {threshold: coggan::constants::FEMALE_THRESHOLD, five_min: coggan::constants::FEMALE_FIVE_MIN, one_min: coggan::constants::FEMALE_ONE_MIN, five_sec: coggan::constants::FEMALE_FIVE_SEC },
    };

    let threshold_s = coggan::PowerValue{ data: curves.threshold, wpkg: threshold/weight, system: String::from("THRESHOLD")};

    let five_min_s = coggan::PowerValue{ data: curves.five_min, wpkg: five_min/weight, system: String::from("FIVE MINUTE")};

    let one_min_s = coggan::PowerValue{ data: curves.one_min, wpkg: one_min/weight, system: String::from("ONE MINUTE")};

    let five_sec_s = coggan::PowerValue{ data: curves.five_sec, wpkg: five_sec/weight, system: String::from("FIVE SECOND")};

    let improvements = curves.get_improvements(weight, threshold, five_min, one_min, five_sec);

    format!("{}<br/>{}<br/>{}<br/>{} && {}", threshold_s.category(), five_min_s.category(), one_min_s.category(), five_sec_s.category(), improvements)
}

#[wasm_bindgen]
pub fn calculate_hr_zones(hr_max: f32, hr_rest: f32) -> String {
    let mut hr = HRZones::default();
    hr.calc_zones(hr_max, hr_rest)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power() {
        let powercalc = calculate_power(84., 342., 431., 806., 1304., true);
        assert_eq!(powercalc.lines().next().unwrap(), String::from("At 4.07 WPKG, your THRESHOLD power is between Cat 3 Level 3 and Cat 3 Level 4<br/>At 5.13 WPKG, your FIVE MINUTE power is between Cat 2 Level 1 and Cat 2 Level 2<br/>At 9.60 WPKG, your ONE MINUTE power is between Cat 1 Level 2 and Cat 1 Level 3<br/>At 15.52 WPKG, your FIVE SECOND power is between Cat 4 Level 6 and Cat 3 / Cat 4 Transition && <tr><th>Category</th><th>Threshold</th><th>Five Minute</th><th>One Minute</th><th>Five Seconds</th></tr><tr><td>World class Level 6</td><td>196W</td><td>207W</td><td>160W</td><td>715W</td></tr><tr><td>World class Level 5</td><td>188W</td><td>199W</td><td>151W</td><td>693W</td></tr><tr><td>World class Level 4</td><td>180W</td><td>190W</td><td>141W</td><td>670W</td></tr><tr><td>World class Level 3</td><td>173W</td><td>181W</td><td>131W</td><td>646W</td></tr><tr><td>World class Level 2</td><td>165W</td><td>173W</td><td>121W</td><td>624W</td></tr><tr><td>World class Level 1</td><td>159W</td><td>164W</td><td>112W</td><td>601W</td></tr><tr><td>Domestic Pro Level 6</td><td>151W</td><td>155W</td><td>102W</td><td>578W</td></tr><tr><td>Domestic Pro Level 5</td><td>144W</td><td>147W</td><td>93W</td><td>556W</td></tr><tr><td>Domestic Pro Level 4</td><td>136W</td><td>138W</td><td>83W</td><td>532W</td></tr><tr><td>Domestic Pro Level 3</td><td>128W</td><td>129W</td><td>73W</td><td>510W</td></tr><tr><td>Domestic Pro Level 2</td><td>121W</td><td>121W</td><td>63W</td><td>487W</td></tr><tr><td>Domestic Pro Level 1</td><td>113W</td><td>112W</td><td>54W</td><td>464W</td></tr><tr><td>Cat 1 Level 6</td><td>106W</td><td>103W</td><td>44W</td><td>442W</td></tr><tr><td>Cat 1 Level 5</td><td>98W</td><td>95W</td><td>35W</td><td>419W</td></tr><tr><td>Cat 1 Level 4</td><td>91W</td><td>86W</td><td>25W</td><td>395W</td></tr><tr><td>Cat 1 Level 3</td><td>84W</td><td>77W</td><td>16W</td><td>373W</td></tr><tr><td>Cat 1 Level 2</td><td>76W</td><td>69W</td><td>5W</td><td>350W</td></tr><tr><td>Cat 1 Level 1</td><td>69W</td><td>60W</td><td>-4W</td><td>327W</td></tr><tr><td>Cat 2 Level 6</td><td>61W</td><td>51W</td><td>-14W</td><td>305W</td></tr><tr><td>Cat 2 Level 5</td><td>54W</td><td>43W</td><td>-23W</td><td>281W</td></tr><tr><td>Cat 2 Level 4</td><td>46W</td><td>34W</td><td>-33W</td><td>258W</td></tr><tr><td>Cat 2 Level 3</td><td>39W</td><td>25W</td><td>-42W</td><td>236W</td></tr><tr><td>Cat 2 Level 2</td><td>31W</td><td>17W</td><td>-53W</td><td>213W</td></tr><tr><td>Cat 2 Level 1</td><td>23W</td><td>7W</td><td>-62W</td><td>190W</td></tr><tr><td>Cat 3 Level 6</td><td>17W</td><td>-1W</td><td>-72W</td><td>167W</td></tr><tr><td>Cat 3 Level 5</td><td>9W</td><td>-10W</td><td>-81W</td><td>144W</td></tr><tr><td>Cat 3 Level 4</td><td>2W</td><td>-19W</td><td>-91W</td><td>121W</td></tr><tr><td>Cat 3 Level 3</td><td>-6W</td><td>-27W</td><td>-100W</td><td>99W</td></tr><tr><td>Cat 3 Level 2</td><td>-14W</td><td>-36W</td><td>-110W</td><td>76W</td></tr><tr><td>Cat 3 Level 1</td><td>-21W</td><td>-45W</td><td>-120W</td><td>53W</td></tr><tr><td>Cat 3 / Cat 4 Transition</td><td>-29W</td><td>-53W</td><td>-130W</td><td>30W</td></tr><tr><td>Cat 4 Level 6</td><td>-36W</td><td>-62W</td><td>-139W</td><td>7W</td></tr><tr><td>Cat 4 Level 5</td><td>-44W</td><td>-71W</td><td>-149W</td><td>-15W</td></tr><tr><td>Cat 4 Level 4</td><td>-51W</td><td>-79W</td><td>-158W</td><td>-38W</td></tr><tr><td>Cat 4 Level 3</td><td>-58W</td><td>-88W</td><td>-168W</td><td>-62W</td></tr><tr><td>Cat 4 Level 2</td><td>-66W</td><td>-97W</td><td>-178W</td><td>-84W</td></tr><tr><td>Cat 4 Level 1</td><td>-73W</td><td>-105W</td><td>-188W</td><td>-107W</td></tr><tr><td>Cat 4 / Cat 5 Transition</td><td>-81W</td><td>-114W</td><td>-197W</td><td>-130W</td></tr><tr><td>Cat 5 Level 6</td><td>-88W</td><td>-123W</td><td>-207W</td><td>-152W</td></tr><tr><td>Cat 5 Level 5</td><td>-96W</td><td>-131W</td><td>-216W</td><td>-175W</td></tr><tr><td>Cat 5 Level 4</td><td>-103W</td><td>-140W</td><td>-226W</td><td>-199W</td></tr><tr><td>Cat 5 Level 3</td><td>-111W</td><td>-149W</td><td>-236W</td><td>-221W</td></tr><tr><td>Cat 5 Level 2</td><td>-119W</td><td>-157W</td><td>-246W</td><td>-244W</td></tr><tr><td>Cat 5 Level 1</td><td>-125W</td><td>-166W</td><td>-255W</td><td>-267W</td></tr><tr><td>Recreational Level 6</td><td>-133W</td><td>-175W</td><td>-265W</td><td>-289W</td></tr><tr><td>Recreational Level 5</td><td>-140W</td><td>-183W</td><td>-274W</td><td>-313W</td></tr><tr><td>Recreational Level 4</td><td>-148W</td><td>-192W</td><td>-284W</td><td>-335W</td></tr><tr><td>Recreational Level 3</td><td>-156W</td><td>-201W</td><td>-294W</td><td>-358W</td></tr><tr><td>Recreational Level 2</td><td>-163W</td><td>-209W</td><td>-303W</td><td>-381W</td></tr><tr><td>Recreational Level 1</td><td>-171W</td><td>-218W</td><td>-313W</td><td>-404W</td></tr><tr><td>Untrained Level 2</td><td>-178W</td><td>-227W</td><td>-322W</td><td>-427W</td></tr><tr><td>Untrained Level 1</td><td>-186W</td><td>-235W</td><td>-332W</td><td>-450W</td></tr>"));
    }
}