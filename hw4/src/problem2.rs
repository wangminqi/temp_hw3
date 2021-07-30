

pub fn sum(input: &[u32]) -> Option<u32> {
    let mut temp = 0u32;
    for i in input {
        let key = temp.checked_add(*i);
        if key.is_none() {return None};
        // println!("sum part is {:?}",*i);
        temp = key.unwrap();
        // println!("sum part is {:?}", temp);
    };
    Some(temp)
}