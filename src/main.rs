use transform::transform;
fn main() {
    let hgt_file="../data/hgt.txt";
    // let slp_file="../data/slp.txt";
    let mut hgt=transform::read_file(hgt_file,19);
    // let slp=transform::read_file(slp_file,33);
    // hgt.push(slp[0].clone());
    let p:Vec<f64>=(0..).map(|x| 100.0+(x as f64)*50.0).take(19).collect();
    let x=transform::CoordinateSystem::new(p,hgt);
    let sigma=x.sigma();
    let (p_vec,sigma_vec)=x.change_sigma(11);
    println!("{:?}",&p_vec);
    let mut sigma_vec_iter=sigma_vec.iter();
    let sigma_arr=x.get_changed_sigma_data(p_vec);
    let _=sigma_arr.iter()
    .map(|x|transform::write_file(x,0,19,&format!("../data/result/{:.1}_layer.txt",sigma_vec_iter.next().unwrap())))
    .collect::<Vec<_>>();
    // println!("{:?}",(0..11).map(|x| (x as f64)*1.0/11.0).collect::<Vec<f64>>());
}
