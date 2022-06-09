/// lattice balun calculation library
/// 
///


pub mod lattice_balun {
    use core::f64::consts::PI;

    // Impedance transformation z1
    fn z1(r1: f64, r2: f64) -> f64 {
        (r1*r2).sqrt()
    }

    // radial frequency, omega = 2*π*f1 
    // units are (radians/s)
    fn omega1(f1: f64) -> f64 {
        2.0 * PI * f1
    }

    // Capacitance Calculation (F)
    pub fn c_from_z1(r1: f64, r2: f64, f1: f64) -> f64 {
            let impedance = z1(r1, r2);
            let omega = omega1(f1);
            1.0/(omega*impedance)
    }

    // Inductance Calculation (H)
    pub fn l_from_z1(r1: f64, r2: f64, f1: f64) -> f64 {
            let impedance = z1(r1, r2);
            let omega = omega1(f1);
            impedance/omega
    }

    mod tests {
        use more_asserts as ma;
        #[test]
        fn test_z1() {
            let expected = 70.71067811865476;
            let tol = 1e-6;
            let result = super::z1(100.0,50.0);
            ma::assert_le!{(result-expected).abs(), tol};
        }

        #[test]
        fn test_omega1() {
            let expected = 300.0e6*3.141592653589793*2.0;
            let tol = 1e-4;
            let result = super::omega1(300.0e6);
            ma::assert_le!{(result-expected).abs(), tol};
        }

        #[test]
        fn test_CfromZ1() {
            let f0 = 300e6; // Hz
            let r1 = 50.0; // ohms
            let r2 = 100.0; // ohms
            let c1 = super::c_from_z1(r1, r2, f0);
            let expected = 7.502635967975885e-12;
            let tol = 1e-4;
            ma::assert_le!{(c1-expected).abs(),tol};
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    //#[test]
    //fn test_z1() {
    //    let result = z1(100.0,50.0);
    //    assert_eq!(result, 2.0);
    //}
}
