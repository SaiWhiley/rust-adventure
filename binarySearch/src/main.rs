fn main() {
    println!("Hello, world!");



}


fn CreateNumbers(size: i32) -> Vec<i32>{
    let A = vec![];

    for x in 0..size {
        A[x] = x;
    }
    return A;
}

// fn RecursiveBinary(A: Vec<i32>, target: i32, low: i32, high: i32) -> i32 {
//     if high < low{
//         return -1;
//     }
//     let mid = (low - high) /2;
//     if target < A[mid] {
//         RecursiveBinary(A, target, low, mid-1);
//     }
//     else if target > A[mid] {
//         RecursiveBinary(A, target, mid+1,high);
//     }
//     else{
//         return mid;
//     }
// }
