extern "C" {
    fn print_app_info();
}

fn main() {
    println!("编译 C 语言库时自定义设置!");
    unsafe {
        print_app_info();
    }
}
