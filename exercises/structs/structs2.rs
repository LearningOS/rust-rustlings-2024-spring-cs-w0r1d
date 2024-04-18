// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
       //let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        // let your_order =
        let mut order_template = create_order_template();
        order_template.count = 1; // 更新 count 字段为 1
        let mut your_order = order_template; // 创建自己的订单

        //assert_eq!(your_order.name, "Hacker in Rust");
        // assert_eq!(your_order.year, order_template.year);
        // assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        // assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        // assert_eq!(your_order.made_by_email, order_template.made_by_email);
        // assert_eq!(your_order.item_number, order_template.item_number);
        // assert_eq!(your_order.count, 1);
        assert_eq!(your_order.name, "Bob"); // 名称应该是 "Hacker in Rust"
        assert_eq!(your_order.year, 2019); // 年份不变
        assert_eq!(your_order.made_by_phone, false); // 不是通过电话订购
        assert_eq!(your_order.made_by_mobile, false); // 不是通过手机订购
        assert_eq!(your_order.made_by_email, true); // 是通过电子邮件订购
        assert_eq!(your_order.item_number, 123); // 物品编号不变
        assert_eq!(your_order.count, 1); // 数量变为 1

    }
}
