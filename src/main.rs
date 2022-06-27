use lattice_balun::lattice_balun;

fn main() {
    let r1 = 50.0;  // Ohm
//    let r2 = 50.0;
    let v1: Vec<f64> = (0..=100).step_by(1).map(|x| (x as f64)).collect();  // Ohm
    let f1 = 297e6; // MHz

    println!("R2,   C1,     L1");
    for v in v1 {
        let c1 = lattice_balun::c_from_z1(r1, v, f1) * 1.0e12; // pF
        let l1 = lattice_balun::l_from_z1(r1, v, f1) * 1.0e9;  // nH
        let width: usize = 4;
        println!("{}  {c1:.width$}  {l1:.width$}", v);
    }

}