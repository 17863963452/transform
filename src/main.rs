use transform::transform;
fn main() {
    let hgt_file="../data/hgt.txt";
    // let slp_file="../data/slp.txt";
    let hgt=transform::read_file(hgt_file);
    // let slp=transform::read_file(slp_file);
    let p:Vec<f64>=(0..).map(|x| 100.0+(x as f64)*50.0).take(19).collect();
    let x=transform::CoordinateSystem::new(p,hgt);
    // let sigma=x.sigma();
    let map_system=x.change_sigma(11);
    println!("{:?}",x.get_changed_sigma_data(map_system));
}
