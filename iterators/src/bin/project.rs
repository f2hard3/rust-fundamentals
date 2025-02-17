#![allow(unused, dead_code)]

use std::{
    collections::HashMap,
    env::{self, args},
    vec,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug, Clone)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();
    println!("blender_orders: {blender_orders:#?}");

    let total_microwave_orders = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("total_microwave_orders: {total_microwave_orders:#?}");

    let user_quantity = args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);

    let orders_by_quantity = orders
        .iter()
        .filter(|order| order.quantity >= user_quantity)
        .collect::<Vec<&CustomerOrder>>();
    println!("orders_by_quantity: {orders_by_quantity:#?}");

    let product_quantities =
        orders
            .iter()
            .filter(|order| !order.shipped)
            .fold(HashMap::new(), |mut acc, curr| {
                let entry = acc.entry(&curr.product).or_insert(0);
                *entry += curr.quantity;
                acc
            });
    println!("product_quantities: {product_quantities:#?}");

    if let Some(order) = orders.iter_mut().find(|order| !order.shipped) {
        order.shipped = true;
        println!("first_unshipped: {:#?}", order);
    } else {
        println!("No unshipped order left");
    }
    println!("{orders:#?}");

    // let mut customers = orders
    //     .iter()
    //     .zip(customer_ids_by_order)
    //     .fold(HashMap::new(), |mut acc, (order, customer_id)| {
    //         let pair = acc.entry(customer_id).or_insert(Vec::new());
    //         pair.push(order.clone());
    //         acc
    //     })
    //     .iter()
    //     .map(|(id, orders)| Customer {
    //         id: *id,
    //         orders: orders.clone(),
    //     })
    //     .collect::<Vec<Customer>>();
    // customers.sort_by_key(|customer| customer.id);
    // println!("customers: {:#?}", customers)

    let mut customers: Vec<Customer> = Vec::new();
    for (i, val) in customer_ids_by_order.iter().enumerate() {
        let order = orders[i].clone();
        if let Some(customer) = customers.iter_mut().find(|customer| customer.id == *val) {
            customer.orders.push(order);
        } else {
            customers.push(Customer {
                id: *val,
                orders: vec![order],
            })
        }
    }
    customers.sort_by_key(|customer| customer.id);
    println!("customers: {customers:#?}");
}
