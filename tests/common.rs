use std::cell::RefCell;

fn test_output(data: &str, expected: &str) {
    let data = data
        .chars()
        .filter_map(bf::Instruction::from_char)
        .collect::<Vec<_>>();

    let s = RefCell::new(String::new());
    let mut vm = bf::Interpreter::new();
    let output_func = |c| {
        //print!("{}", c as char);
        s.borrow_mut().push(char::from(c));
    };
    vm.set_output_func(&output_func);
    vm.exec(&data);

    assert_eq!(s.borrow().as_str(), expected);
}

#[test]
fn factorial() {
    test_output(
        include_str!("factorial.bf"),
        "0! = 1\n1! = 1\n2! = 2\n3! = 6\n4! = 24\n5! = 120\n6! = 28\n",
    ); //Requires 16 bit cells for proper answer
}

#[test]
fn squares() {
    test_output(include_str!("squares.bf"), "0\n1\n4\n9\n16\n25\n36\n49\n64\n81\n100\n121\n144\n169\n196\n225\n256\n289\n324\n361\n400\n441\n484\n529\n576\n625\n676\n729\n784\n841\n900\n961\n1024\n1089\n1156\n1225\n1296\n1369\n1444\n1521\n1600\n1681\n1764\n1849\n1936\n2025\n2116\n2209\n2304\n2401\n2500\n2601\n2704\n2809\n2916\n3025\n3136\n3249\n3364\n3481\n3600\n3721\n3844\n3969\n4096\n4225\n4356\n4489\n4624\n4761\n4900\n5041\n5184\n5329\n5476\n5625\n5776\n5929\n6084\n6241\n6400\n6561\n6724\n6889\n7056\n7225\n7396\n7569\n7744\n7921\n8100\n8281\n8464\n8649\n8836\n9025\n9216\n9409\n9604\n9801\n10000\n");
}

#[test]
fn hello_world3() {
    test_output(include_str!("hello_world3.bf"), "Hello, world!\n");
}

#[test]
fn hello_world2() {
    test_output(include_str!("hello_world2.bf"), "Hello World!\n");
}

#[test]
fn hello_world1() {
    test_output(include_str!("hello_world1.bf"), "Hello World!\n");
}

#[test]
fn count_down() {
    test_output(include_str!("count_down.bf"), "9 8 7 6 5 4 3 2 1 0 ");
}

#[test]
fn aids() {
    test_output(
        include_str!("aids.bf"),
        "How are you?I fucked a cheese burger",
    );
}
