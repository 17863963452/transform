//p向sigma坐标系转化模块
pub mod transform{
    use std::io::{prelude::*,Write};
    use std::fs;
    pub fn read_file(file_name:&str,row:usize)->Vec<Vec<Vec<f64>>>{
        let mut f=fs::File::open(file_name).unwrap();
        let mut buf=String::new();
        f.read_to_string(&mut buf).unwrap();
        let vector2:Vec<Vec<f64>>=buf
            .split_terminator('\n')
            .map(|x| x.trim().split(" ").map(|z| z.parse().unwrap()).collect())
            .collect();
        let vector:Vec<Vec<Vec<f64>>>=vector2.iter()
            .map(|x| x.chunks(row).map(|z| z.to_vec()).collect())
            .collect();
        let mut result_vec:Vec<Vec<Vec<f64>>>=(0..(vector[0][0].len()-1))
            .map(|i| vector[..][..][i].clone())
            .collect();
        result_vec
    }
    pub fn write_file(arr:&Vec<Vec<f64>>,start:usize,end:usize,name:&str){
        let mut f=fs::File::create(name).unwrap();
        for i in arr[start..end].iter(){
            for j in i.iter(){
                f.write_all(format!("{:.5}",(j.to_string())).as_bytes()).unwrap();
                f.write_all(" ".as_bytes()).unwrap();
            }
            f.write_all("\n".as_bytes()).unwrap();
        }
    }
    pub struct CoordinateSystem{
        p:Vec<f64>,
        sigma:Vec<f64>,
        p_data:Vec<Vec<Vec<f64>>>,
    }
    impl CoordinateSystem{
        //构造函数
        pub fn new(p:Vec<f64>,p_data:Vec<Vec<Vec<f64>>>)->CoordinateSystem{
            let (up,down)=(p[0],p[p.len()-1]);
            let sigma:Vec<f64>=p.iter().map(|x| (x-up)/(down-up)).collect();
            CoordinateSystem{p:p,sigma:sigma,p_data:p_data}
        }
        pub fn linear_interpolation(p0:f64,p1:f64,p_middle:f64,d0:f64,d1:f64)->f64{
            (d0-d1)*(p_middle-p1)/(p0-p1)+d1
        }
        //二维线性插值函数
        pub fn linear_interpolation_vec(p0:f64,p1:f64,p_middle:f64,d0:Vec<Vec<f64>>,d1:Vec<Vec<f64>>)->Vec<Vec<f64>>{
            let row=d0.len();
            let column=d0[0].len();
            (0..row).map(|r| (0..column).map(|c| CoordinateSystem::linear_interpolation(p0,p1,p_middle,d0[r][c],d1[r][c])).collect()).collect()
        }
        //输入转换到几个sigma层，输出(转换后p的数组,转换后的sigma数组)
        pub fn change_sigma(&self,num:usize)->(Vec<f64>,Vec<f64>){
            let step=1.0/((num-1) as f64);
            let changed_sigma=(0..).take(num).map(|x| (x as f64)*step).collect::<Vec<f64>>();
            let changed_p:Vec<f64>=changed_sigma.iter()
                .map(|x| x*(self.p[self.p.len()-1]-self.p[0])+self.p[0])
                .collect();
            (changed_p,changed_sigma)
        }
        pub fn get_changed_sigma_data(&self,p_vec:Vec<f64>)->Vec<Vec<Vec<f64>>>{
            //p_vec是p坐标系的数组
            //p_vec从小到大排
            //p_middle是需要插值的p层,p0是顶层p,p1是底层p
            let mut result_vec:Vec<Vec<Vec<f64>>>=p_vec
            .iter().skip(1).take(p_vec.len()-2)
            .map(|p_middle|{
                //选出数组p中比p_position大的元素的第一个
                let p1_position:usize=(0..(self.p.len())).filter(|i| self.p[*i]>*p_middle).take(1).collect::<Vec<usize>>()[0];
                println!("{}",self.p[p1_position]);
                let (p0,p1)=(self.p[p1_position-1],self.p[p1_position]);
                CoordinateSystem::linear_interpolation_vec(p0,p1,*p_middle,self.p_data[p1_position-1].clone(),self.p_data[p1_position].clone())})
            .collect();
            result_vec.insert(0,self.p_data[0].clone());
            result_vec.push(self.p_data[self.p.len()-2].clone());
            result_vec
        }
        pub fn p(&self)->&Vec<f64>{
            &self.p
        }
        pub fn sigma(&self)->&Vec<f64>{
            &self.sigma
        }
    }
}
