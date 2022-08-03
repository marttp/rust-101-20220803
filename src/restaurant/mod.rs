// Crates : Modules that produce a library or executable
// Modules : Organize and handle privacy
// Packages : Build, test and share crates
// Paths : A way of naming an item such as a struct, function

mod pizza_order {

  pub struct Pizza {
      pub dough: String,
      pub cheese: String,
      pub topping: String,
  }

  // Implement functionality for the Pizza struct
  impl Pizza {
      pub fn lunch(topping: &str) -> Pizza {
          Pizza {
              dough: String::from("regular dough"),
              cheese: String::from("mozzarella"),
              topping: String::from(topping),
          }
      }
  }

  pub mod help_customer {
      // This function is private
      fn seat_at_table() {
          println!("Customer seated at table");
      }

      pub fn take_order() {
          seat_at_table();
          let cust_pizza: super::Pizza =
              super::Pizza::lunch("veggies");
          serve_customer(cust_pizza);
      }

      fn serve_customer(cust_pizza: super::Pizza){
          println!("The customer is served a regular pizza with {}", cust_pizza.topping);
      }

  }
}

// This is the public function that allows our other file access
pub fn order_food() {
  crate::restaurant::pizza_order::help_customer::take_order();
}
