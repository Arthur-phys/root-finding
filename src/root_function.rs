use crate::Error;

pub struct RootFunction {
    function: Box<dyn Fn(f64) -> f64>,
    pub error_tolerance: f64  
}

impl RootFunction {

    pub fn new(function: impl Fn(f64) -> f64 + 'static, error_tolerance: f64) -> Result<Self,Error> {

        if error_tolerance.is_sign_negative() || error_tolerance == 0.0 {
            return Err(Error::InvalidTolerance);
        }

        Ok(Self {
            function: Box::new(function),
            error_tolerance
        })
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        (self.function)(x)
    }

}

#[cfg(test)]
mod tests {

    use super::RootFunction;

    #[test]
    fn new_root_function() {

        let my_func = |x| x + 2.0;
        let err = 0.001;

        let my_root_func = RootFunction::new(my_func,err);

    }

    #[test]
    fn evaluate_root_func() {

        let my_func = |x| x + 2.0;
        let err = 0.001;

        let my_root_func = RootFunction::new(my_func,err).unwrap();

        let val = my_root_func.evaluate(2.0);
        assert!(val == 4.0);

    }

}