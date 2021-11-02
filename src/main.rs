
fn calculate_sum(num:&[u32])->Option<u32>{
    let mut sum=0;
    let max = std::u32::MAX;
    for i in num{
        if max - sum<*i{
            return None;
        }
        sum+=i;
    }
    Some(sum)

}
fn main() {
    let mut num=vec![2,4,5,7];
    println!("The sum of the array: {:?} is {:?}", num, calculate_sum(&num));
    num=vec![1,4,2,3,1,1111];
    println!("The sum of the array: {:?} is {:?}", num, calculate_sum(&num));
    num=vec![5,111,1111,4294967280];
    println!("The sum of the array: {:?} is {:?}", num, calculate_sum(&num));
}
