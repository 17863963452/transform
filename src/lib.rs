pub mod transform{
    use std::fs::File;
    use std::io::prelude::*;
    pub fn read_file(file_name:&str)->Vec<Vec<Vec<f64>>>{
        let mut f=File::open(file_name).unwrap();
        let mut buf=String::new();
        f.read_to_string(&mut buf).unwrap();
        let vector2:Vec<Vec<f64>>=buf
            .split_terminator('\n')
            .map(|x| x.trim().split(" ").map(|z| z.parse().unwrap()).collect())
            .collect();
        let vector:Vec<Vec<Vec<f64>>>=vector2.iter()
            .map(|x| x.chunks(33).map(|z| z.to_vec()).collect())
            .collect();
        vector
    }
    pub struct CoordinateSystem{
        p:Vec<f64>,
        sigma:Vec<f64>,
        p_data:Vec<Vec<Vec<f64>>>,
    }
    use std::collections::HashMap;
    impl CoordinateSystem{
        pub fn new(p:Vec<f64>,p_data:Vec<Vec<Vec<f64>>>)->CoordinateSystem{
            let (up,down)=(p[0],p[p.len()-1]);
            let sigma:Vec<f64>=p.iter().map(|x| (x-up)/(down-up)).collect();
            CoordinateSystem{p:p,sigma:sigma,p_data:p_data}
        }
        pub fn linear_interpolation(p0:f64,p1:f64,p_middle:f64,d0:f64,d1:f64)->f64{
            (d1-d0)*(p_middle-p0)/(p1-p0)+d0
        }
        pub fn linear_interpolation_vec(p0:f64,p1:f64,p_middle:f64,d0:Vec<Vec<f64>>,d1:Vec<Vec<f64>>)->Vec<Vec<f64>>{
            let row=d0.len();
            let column=d0[0].len();
            (0..row).map(|r| (0..column).map(|c| CoordinateSystem::linear_interpolation(p0,p1,p_middle,d0[r][c],d1[r][c])).collect()).collect()
        }
        pub fn change_sigma(&self,num:usize)->HashMap<String,f64>{
            let step=1.0/((num-1) as f64);
            let changed_sigma=(0..).take(num).map(|x| (x as f64)*step).collect::<Vec<f64>>();
            let changed_p:Vec<f64>=changed_sigma.iter()
                .map(|x| x*(self.p[self.p.len()-1]-self.p[0])+self.p[0])
                .collect();
            let mut map=HashMap::new();
            let _=(0..changed_sigma.len())
                .map(|x| map.insert(format!("{}",changed_sigma[x]),changed_p[x]))
                .collect::<Vec<_>>();
            map
        }
        pub fn get_changed_sigma_data(&self,map_system:HashMap<String,f64>)->Vec<Vec<Vec<f64>>>{
            println!("{:?},{:?}",map_system,self.p);
            map_system.iter()
            .map(|(k,p_middle)|{
                let p1_position:usize=(0..(self.p.len())).filter(|i| self.p[*i]>*p_middle).take(1).collect::<Vec<usize>>()[0];
                let (p0,p1)=(self.p[p1_position-1],self.p[p1_position]);
                match (k){
                    format!("{}",self.p[0])=>CoordinateSystem::linear_interpolation_vec(p0,p1,*p_middle,self.p_data[p1_position-1].clone(),self.p_data[p1_position].clone()),
                    format!("{}",self.p[self.p.len()-1])=>CoordinateSystem::linear_interpolation_vec(p0,p1,*p_middle,self.p_data[p1_position-1].clone(),self.p_data[p1_position].clone()),
                    _=>,
                }
            })
            .collect()
        }
        pub fn p(&self)->&Vec<f64>{
            &self.p
        }
        pub fn sigma(&self)->&Vec<f64>{
            &self.sigma
        }
    }
}
