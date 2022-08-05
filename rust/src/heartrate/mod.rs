use std::ops::Index;

pub struct HRZones {
    pub one: [i32; 2],
    pub two: [i32; 2],
    pub three: [i32; 2],
    pub four: [i32; 2],
    pub five: [i32; 2],
}

impl HRZones {
    pub fn default() -> HRZones {
        HRZones {
            one: [0,0],
            two: [0,0],
            three: [0,0],
            four: [0,0],
            five: [0,0]
        }
    }

    pub fn calc_zones(&mut self, hr_max: f32, hr_rest: f32) -> String {
        let hrr = hr_max - hr_rest;
        let mut zones = String::from("");
        self.calc_one(hrr, hr_rest);
        self.calc_two(hrr, hr_rest);
        self.calc_three(hrr, hr_rest);
        self.calc_four(hrr, hr_rest);
        self.calc_five(hrr, hr_rest);
        for i in 1..6 {
            zones.push_str(&format!("<tr><td>Zone {}</td><td>{:.0}</td><td>{:.0}</td></tr>",i, self[i][0], self[i][1]));
        }
        zones
    }

    fn calc_one(&mut self, hrr: f32, hr_rest: f32){
        self.one[0] = hr_rest as i32;
        self.one[1] = ((0.6 * hrr) + hr_rest) as i32;
    }

    fn calc_two(&mut self, hrr: f32, hr_rest: f32){
        self.two[0] = ((0.6 * hrr) + hr_rest + 1.) as i32;
        self.two[1] = ((0.7 * hrr) + hr_rest) as i32;
    }

    fn calc_three(&mut self, hrr: f32, hr_rest: f32){
        self.three[0] = ((0.7 * hrr) + hr_rest + 1.) as i32;
        self.three[1] = ((0.8 * hrr) + hr_rest) as i32;
    }

    fn calc_four(&mut self, hrr: f32, hr_rest: f32){
        self.four[0] = ((0.8 * hrr) + hr_rest + 1.) as i32;
        self.four[1] = ((0.9 * hrr) + hr_rest) as i32;
    }

    fn calc_five(&mut self, hrr: f32, hr_rest: f32,){
        self.five[0] = ((0.9 * hrr) + hr_rest + 1.) as i32;
        self.five[1] = (hrr + hr_rest) as i32;
    }

}

impl Index<usize> for HRZones {
    type Output = [i32];

    fn index(&self, index:usize) -> &Self::Output {
        match index {
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            5 => &self.five,
            _ => panic!("Index out of Range"),
        }
    }
}