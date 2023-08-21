use crate::RootFunction;
use crate::Error;
use crate::methods::utils::sgn;

impl RootFunction {

    pub fn bisection(&self, interval: [f64; 2]) -> Result<f64, Error> {

        let mut a = interval[0];
        let mut b = interval[1];

        if sgn(self.evaluate(a)) * sgn(self.evaluate(b)) > 0.0 || a >= b {
            return Err(Error::InvalidInterval);
        }

        let number_of_iterations = ((b - a) / self.error_tolerance).log2().ceil();
        let mut i = 0;

        while i < number_of_iterations as u64 {
            
            let c = a + (b - a) / 2.0;
            let sgn_c = sgn(self.evaluate(c));
            let sgn_a = sgn(self.evaluate(a));

            if sgn_c == 0.0 {
                return Ok(c);
            } else if sgn_c * sgn_a < 0.0 {
                b = c;
            } else {
                a = c;
            }

            i += 1;
        }

        Ok((a + b) / 2.0)

    }

}

#[cfg(test)]
mod tests {

    use crate::RootFunction;

    #[test]
    fn calc_simple() {

        let my_func = |x| x + 2.0;
        let err = 0.001;

        let my_root_func = RootFunction::new(my_func,err).unwrap();

        let probably_m2 = my_root_func.bisection([-3.0,3.0]).unwrap();

        assert!( probably_m2 <= -1.5 && probably_m2 >= -2.5 );


    }

    #[test]
    fn calc_harder() {

        let my_func = |x: f64| x.cos();
        let err = 0.001;

        let my_root_func = RootFunction::new(my_func,err).unwrap();

        let probably_pi = my_root_func.bisection([1.0,4.0]).unwrap();

        assert!( probably_pi <= 1.65 && probably_pi >= 1.5 );

    }

}