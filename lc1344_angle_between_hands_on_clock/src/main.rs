
/* 
Given two numbers, hour and minutes, return the smaller angle (in degrees) formed between the hour and the minute hand.

Answers within 10-5 of the actual value will be accepted as correct.

Constraints:

    1 <= hour <= 12
    0 <= minutes <= 59

 */

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        const MINUTES_TO_DEGREES: f64 = 360.0 / 60.0;
        const HOURS_TO_DEGREES: f64 = 360.0 / 12.0;

        let abs_angle_minute: f64 = minutes as f64 * MINUTES_TO_DEGREES;
        let abs_angle_hour: f64 = ((hour % 12 ) as f64 + (minutes as f64 / 60.0)) * HOURS_TO_DEGREES;

        let diff = abs_angle_hour - abs_angle_minute;
        if diff.abs() > 180.0 {
            (diff.abs() - 360.0).abs()
        } else {
            diff.abs()
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::angle_clock(12, 30), 165.0);
    assert_eq!(Solution::angle_clock(3, 30), 75.0);
    assert_eq!(Solution::angle_clock(3, 15), 7.5);
    assert_eq!(Solution::angle_clock(9, 0), 90.0);
    assert_eq!(Solution::angle_clock(1, 57), 76.5);
    
}
