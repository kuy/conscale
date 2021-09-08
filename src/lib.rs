pub fn map_fn(domain: (u16, u16), codomain: (i32, i32)) -> impl Fn(u16) -> i32 {
    // TODO: check pre-conditions

    move |input| {
        let (input_min, input_max) = domain;
        let (output_min, output_max) = codomain;

        if input <= input_min {
            return output_min;
        }

        if input_max <= input {
            return output_max;
        }

        let ratio = (input - input_min) as f64 / (input_max - input_min) as f64;
        ((output_max - output_min) as f64 * ratio + output_min as f64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_fn() {
        let map = map_fn((0, 100), (0, 100));
        assert_eq!(0, map(0));
        assert_eq!(1, map(1));
        assert_eq!(100, map(100));
        assert_eq!(100, map(101));

        let map = map_fn((0, 100), (0, 255));
        assert_eq!(0, map(0));
        assert_eq!(2, map(1));
        assert_eq!(127, map(50));
        assert_eq!(255, map(100));
        assert_eq!(255, map(101));

        let map = map_fn((10, 110), (-127, 127));
        assert_eq!(-127, map(9));
        assert_eq!(-127, map(10));
        assert_eq!(0, map(60));
        assert_eq!(-50, map(40));
        assert_eq!(50, map(80));
        assert_eq!(127, map(110));
        assert_eq!(127, map(111));

        let map = map_fn((0, 10000), (-16777216, -1));
        assert_eq!(-16777216, map(0));
        assert_eq!(-16000430, map(463));
        assert_eq!(-13000664, map(2251));
        assert_eq!(-1, map(10000));
        assert_eq!(-1, map(10001));
    }
}
