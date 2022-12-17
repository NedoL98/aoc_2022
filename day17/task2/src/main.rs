fn main() {
    println!("1540634005751");
}

// There is a loop that starts in position 650, has length 1735 
// and increases height by 2673 every time. 
//
// Thus the answer is 
// ((10**12 - 651) // 1735) * 2673 + ans_for_loop_suffix(651) + ans_for_loop_prefix((10**12 - 651) % 1735) =
// 1540634002875 + ans_for_loop_suffix(651) + ans_for_loop_prefix(1224) =
// 1540634002875 + ans_for_loop_suffix(1224 + 651) =
// 1540634002875 + ans_original_problem(1875) =
// 1540634002875 + 2876 =
// 1540634005751
