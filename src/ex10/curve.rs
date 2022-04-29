/// ## Map on a space filling curve
///
/// Find the point at coordinate [x,y] on the space filling curve (here z-order).
///
/// The 2 point are 16 bits unsigned integer the result is a 32 bit unsigned integer.
///
/// The return value of the fuction is a float between 0 and 1 corresponding to the
/// value of the result / u32::MAX
///
/// ### Z-Order
///
/// exemple for :
/// x = 51 <=> 0b110011 x -> from x
/// y = 47 <=> 0b101111	y -> from y
///
/// the point on the z curve can be found at
/// y1 x1 y0 x0 y1 x0 y1 x0 y1 x1 y1 x1
/// -> 0b110110101111 <=> 3503
///
/// #### exemple :
///	|   y\x   |  **0** |  **1** | **10** | **11** | **100** |
/// |:-------:|:------:|:------:|:------:|:------:|:-------:|
/// |  **0**  |    0   |    1   |   100  |   101  |  10000  |
/// |  **1**  |   10   |   11   |   110  |   111  |  10010  |
/// |  **10** |  1000  |  1001  |  1100  |  1101  |  11000  |
/// |  **11** |  1010  |  1011  |  1110  |  1111  |  11010  |
/// | **100** | 100000 | 100001 | 100100 | 100101 |  110000 |
pub fn map(x: u16, y: u16) -> f64 {
    let mut result: u32 = 0;
    let mut current_x: u32;
    let mut current_y: u32;
    for n in 0..16 {
        current_x = ((x as u32) >> n) & 1;
        current_y = ((y as u32) >> n) & 1;

        result += (current_x << (n * 2)) + (current_y << (n * 2 + 1));
    }
    // println!("x{:b} <=> ({}), y{:b} <=> ({}), {:b} <=> {}",x, x, y, y, result, result);
    result as f64 / u32::MAX as f64
}
