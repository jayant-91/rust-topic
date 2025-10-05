
// This function dont give err
// cause i32 impliment copy and std::ops::AddAssign trait

pub fn sum_of_elements(nums: &[i32]) -> i32 {
    let mut sum = nums[0];  //value will copy
    for i in 1..nums.len() {
        sum += nums[i]; //value will add
    }

    sum
}


//This function give error 
// because T not imliment Copy and AddAssign trait
// we have to add Trait Bounds for get ride of this error

pub fn sum_of_elements_generic<T>(nums: &[T]) -> T
where
    T: Copy + std::ops::AddAssign
{
    let mut sum = nums[0];  //Value will not copy
    for i in 1..nums.len() {
        sum += nums[i]; //value will not add
    }
    
    sum
}


fn main() {
    let my_nums = [1, 2, 3, 4, 5];
    let your_nums = [6, 5, 4, 3, 2];
    let my_sum = sum_of_elements(&my_nums);
    let your_sum = sum_of_elements(&your_nums);

    let my_i64_nums: [i64; 5] = [1, 2, 3, 4, 5];
    let my_i64_sum = sum_of_elements_generic::<i64>(&my_i64_nums);
    println!("sums: {my_sum}, {your_sum}, {my_i64_sum}");

    let v: Vec<i64> = vec![1, 2, 3];
}
