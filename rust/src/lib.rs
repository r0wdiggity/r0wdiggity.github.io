use wasm_bindgen::prelude::*;

mod constants;

pub trait BinarySearch {
    fn search(&self) -> usize;
}

pub trait StringBuilder {
    fn category(&self) -> String;
    fn helper(&self, index: usize) -> String;
}

struct Curves {
    threshold: [f32; 52],
    five_min: [f32; 52],
    one_min: [f32; 52],
    five_sec: [f32; 52],
}
impl Curves {
    fn get_improvements(&self, weight: f32, threshold: f32, five_min: f32, one_min: f32, five_sec: f32) -> String {
        let mut improvements = String::from("<tr><th>Category</th><th>Threshold</th><th>Five Minute</th><th>One Minute</th><th>Five Seconds</th></tr>");
        for i in 0..constants::CATEGORIES.len() {
            let t_i = (self.threshold[i] - (threshold / weight)) * weight;
            let f_i = (self.five_min[i] - (five_min / weight)) * weight;
            let o_i = (self.one_min[i] - (one_min / weight)) * weight;
            let s_i = (self.five_sec[i] - (five_sec / weight)) * weight;
            improvements.push_str(&format!("<tr><td>{}</td><td>{:.0}W</td><td>{:.0}W</td><td>{:.0}W</td><td>{:.0}W</td></tr>", constants::CATEGORIES[i], t_i, f_i, o_i, s_i));
        }
        improvements
    }
}

struct PowerValue {
    data: [f32; 52],
    wpkg: f32,
    system: String,
}
impl BinarySearch for PowerValue {
    fn search(&self) -> usize {
        // This function will find the closest value in the array and return its index
        let mut low: i8 = 0;
        let mut high: i8 = self.data.len() as i8;
        let mut idx = 0;
        while low <= high {
            let mid = ((high - low) / 2) + low;
            idx = mid as usize;
            let val = &self.data[idx];

            if val == &self.wpkg {
                return idx
            }
            if val < &self.wpkg {
                high = mid - 1;
            }
            if val > &self.wpkg {
                low = mid + 1;
            }
        }
        idx
    }
}
impl StringBuilder for PowerValue {
    fn category(&self) -> String {
        let index = self.search();
        match index {
            0 => format!("At {:.2} WPKG, your {} power is performaing at {}", self.wpkg, self.system, constants::CATEGORIES[index]),
            51 => format!("At {:.2} WPKG, your {} power is considered {}", self.wpkg, self.system, constants::CATEGORIES[index]),
            _ => self.helper(index)
        }
    }
    fn helper(&self, index: usize) -> String {
        if self.wpkg < constants::MALE_THRESHOLD[index] {
            format!("At {:.2} WPKG, your {} power is between {} and {}",
            self.wpkg,
            self.system,
            constants::CATEGORIES[index + 1],
            constants::CATEGORIES[index])
        } else {
            format!("At {:.2} WPKG, your {} power is between {} and {}",
            self.wpkg,
            self.system,
            constants::CATEGORIES[index],
            constants::CATEGORIES[index - 1])
        }
    }
}

#[wasm_bindgen]
pub fn calculate_power(weight: f32, threshold: f32, five_min: f32, one_min: f32, five_sec: f32, gender: bool) -> String {
    let curves: Curves = match gender {
        true => Curves {threshold: constants::MALE_THRESHOLD, five_min: constants::MALE_FIVE_MIN, one_min: constants::MALE_ONE_MIN, five_sec: constants::MALE_FIVE_SEC },
        false =>  Curves {threshold: constants::FEMALE_THRESHOLD, five_min: constants::FEMALE_FIVE_MIN, one_min: constants::FEMALE_ONE_MIN, five_sec: constants::FEMALE_FIVE_SEC },
    };

    let threshold_s = PowerValue{ data: curves.threshold, wpkg: threshold/weight, system: String::from("THRESHOLD")};

    let five_min_s = PowerValue{ data: curves.five_min, wpkg: five_min/weight, system: String::from("FIVE MINUTE")};

    let one_min_s = PowerValue{ data: curves.one_min, wpkg: one_min/weight, system: String::from("ONE MINUTE")};

    let five_sec_s = PowerValue{ data: curves.five_sec, wpkg: five_sec/weight, system: String::from("FIVE SECOND")};

    let improvements = curves.get_improvements(weight, threshold, five_min, one_min, five_sec);

    format!("{}<br/>{}<br/>{}<br/>{} && {}", threshold_s.category(), five_min_s.category(), one_min_s.category(), five_sec_s.category(), improvements)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power() {
        let powercalc = calculate_power(84., 342., 431., 806., 1304., true);
        assert_eq!(powercalc.lines().next().unwrap(), String::from("At 4.07 WPKG, your threshold power is between Cat 3 Level 3 and Cat 3 Level 4"));
    }
}