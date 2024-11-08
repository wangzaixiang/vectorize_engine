use vector_example1::sort2::merge_sort;

fn main() {

    // let mut p1 = vec! [5, 21, 44, 62, 69, 150, 193, 217, 298, 342, 370, 389, 395, 471, 494, 533, 560, 568, 575, 577, 612, 622, 660, 725, 762, 783, 830, 896, 916, 918, 922, 937];
    // let mut p2 = vec! [23, 43, 59, 64, 83, 91, 122, 123, 145, 150, 193, 238, 291, 334, 366, 375, 377, 389, 402, 405, 484, 495, 516, 526, 530, 547, 554, 578, 589, 592, 603, 612, 615, 638, 648, 651, 675, 689, 723, 725, 736, 745, 751, 757, 768, 770, 772, 779, 788, 807, 832, 840, 844, 849, 864, 879, 933, 936, 946, 953, 957, 969, 992, 999];

    // let mut p1 = vec! [5, 10, 18, 46, 48, 87, 96, 98, 122, 127, 133, 134, 152, 181, 194, 217, 239, 289, 293, 293, 294, 309, 314, 315, 320, 320, 329, 344, 348, 355, 366, 381, 420, 434, 440, 481, 491, 499, 506, 512, 522, 527, 585, 585, 591, 613, 616, 640, 653, 657, 671, 679, 682, 685, 689, 693, 709, 713, 720, 738, 746, 757, 761, 785, 789, 793, 795, 803, 806, 811, 846, 856, 870, 897, 918, 939, 958, 975, 978, 982];
    // let mut p2 = vec! [4, 33, 36, 64, 104, 137, 234, 242, 254, 354, 375, 406, 496, 502, 539, 545, 550, 562, 562, 577, 633, 650, 671, 671, 683, 697, 700, 710, 710, 767, 811, 818, 833, 869, 880, 880, 884, 892, 894, 896, 912, 917, 925, 933, 965, 968, 975, 996];

    let mut p1 = vec! [23, 27, 30, 46, 70, 84, 105, 133, 143, 151, 156, 157, 162, 164, 166, 174, 178, 213, 246, 249, 249, 257, 272, 275, 278, 281, 289, 290, 316, 326, 335, 339, 347, 374, 375, 379, 394, 408, 434, 445, 451, 461, 464, 510, 521, 530, 534, 550, 565, 574, 590, 593, 599, 602, 607, 643, 649, 653, 660, 682, 682, 687, 695, 739, 762, 810, 833, 835, 866, 874, 891, 892, 911, 941, 952, 966, 974, 975, 981, 981];
    let mut p2 = vec! [13, 57, 128, 154, 157, 161, 161, 220, 227, 242, 283, 359, 362, 374, 419, 421, 441, 491, 499, 502, 509, 522, 560, 561, 574, 645, 664, 693, 710, 711, 724, 795, 798, 800, 804, 806, 851, 861, 911, 915, 916, 917, 920, 926, 948, 962, 982, 996];


    let mut temp = vec![];
    temp.extend_from_slice(&p1);
    temp.extend_from_slice(&p2);
    temp.sort();

    debug_print("p1", &p1);
    debug_print("p2", &p2);
    debug_print("temp", &temp);

    //temp.clear();

    let mut temp2 = vec![0u32; (p1.len()/16 + p2.len()/16 + 1)/2 * 16 ];  //  (len1/16 + len2/16 +1) / 2 * 16
    merge_sort(&mut p1, &mut p2, &mut temp2);
    println!("after merge sort");
    debug_print("p1", &p1);
    debug_print("p2", &p2);

    assert_eq!(p1, &temp[0..p1.len()]);
    assert_eq!(p2, &temp[p1.len()..]);

}

fn debug_print(label: &str, v: &Vec<u32>){

    println!("{:10}",  label);
    for i in (0..v.len()).step_by(16) {
        print!("\t");
        for j in i..i+16 {
            print!("{:4} ", v[j]);
        }
        println!();
    }
    println!("\n");
}