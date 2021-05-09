use transform::transform::CoordinateSystem;
#[test]
fn test_inter(){
    assert!((CoordinateSystem::linear_interpolation(250.0,500.0,350.0,-5.0,10.0)-1.0)<1e-5);
}
