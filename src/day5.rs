

#[derive(Clone)]
struct PageInfo{
    order: Vec<Vec<i32>>,
    page_number: Vec<Vec<i32>>
}

fn split_data(input_str: String) -> PageInfo{
    let input_parts:Vec<String> = input_str.split("\n\n").map(|v| v.to_string()).collect();
    let order = &input_parts[0];
    let page_number = &input_parts[1];
    let order_vec = order.split("\n")
        .map(|row| row.split("|").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

        let mut page_number = page_number.split("\n").collect::<Vec<&str>>();
    page_number.pop();
    let page_number_vec: Vec<Vec<i32>> = page_number.iter()
        .map(|row| row.split(",").map(|v: &str| v.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let page_info = PageInfo{order:order_vec, page_number:page_number_vec};
    return page_info;
}

pub fn part_one(input_str: String){
    let mut final_val = 0;
    let page_info = split_data(input_str);
    for numb in page_info.page_number{
        let mut correct_order = true;
        for order_row in page_info.order.clone(){
            let first_order: Vec<usize> = numb.iter().position(|&val| val==order_row[0]).into_iter().collect::<Vec<usize>>();
            let second_order: Vec<usize> = numb.iter().position(|&val| val==order_row[1]).into_iter().collect::<Vec<usize>>();
            // println!("T {:?}, {:?}, {:?}-{:?}",numb, order_row, first_order, second_order);
            if first_order.len() != 0 && second_order.len() != 0{
                if first_order[0] > second_order[0] {
                    correct_order = false;

                }
            }
        }
        if correct_order{
            final_val += numb[numb.len()/2]
        }
    }
    println!("{}",final_val);
}

pub fn part_two(input_str: String){
    let mut final_val = 0;
    let mut page_info = split_data(input_str);
    for (i, numb) in page_info.page_number.iter().enumerate(){
        let mut new_num = numb.clone();
        let mut correct_order = false;
        for order_row in page_info.order.clone(){
            println!("{:?} {:?}", new_num, order_row);
            let first_order: Vec<usize> = new_num.iter().position(|&val| val==order_row[0]).into_iter().collect::<Vec<usize>>();
            let second_order: Vec<usize> = new_num.iter().position(|&val| val==order_row[1]).into_iter().collect::<Vec<usize>>();
            // println!("T {:?}, {:?}, {:?}-{:?}",new_num, order_row, first_order, second_order);
            if first_order.len() != 0 && second_order.len() != 0{
                // println!("Numb {:?}", new_num);
                if first_order[0] > second_order[0] {
                    println!("old {:?} {:?}-{:?} {:?}", new_num, first_order, second_order, order_row);
                    let f_val = new_num[first_order[0]];
                    let s_val = new_num[second_order[0]];
                    new_num[first_order[0]] = s_val;
                    new_num[second_order[0]] = f_val;
                    println!("New {:?}", new_num);
                    correct_order = true
                }
            }
        }
        if correct_order{
            final_val += new_num[new_num.len()/2]
        }
    }
    println!("{}",final_val);
}