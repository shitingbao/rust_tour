pub fn example_vec() {
    println!("thsi is vec");

    let mut vec_list = Vec::new();
    vec_list.push(33);

    let da = &vec_list[0];

    println!("{da}");
    vec_list.push(44);

    let res = vec_list.pop();

    println!("{:?}", res);

    let mut vecs = vec![12, 14, 15, 67];
    let data = &vecs[0];
    println!("data:{:?}", data);

    let vdata = vecs.get(2);
    println!("three:{:?}", vdata);

    for da in &mut vecs {
        println!("{da}")
    }
}
