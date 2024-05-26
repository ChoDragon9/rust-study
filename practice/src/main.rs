use std::collections::HashMap;

fn main() {
  let mut orders: HashMap<i32, Car> = HashMap::new();

  let mut order = 1;
  let mut car: Car;
  let mut miles = 0;

  while order <= 11 {
    car = car_factory(order, miles);
    orders.insert(order, car);
    println!("{}: {:?}", order, orders.get(&order));

    miles = if miles == 2100 { 
      0
    } else {
      miles + 700
    };

    order += 1;
  }
}

#[derive(PartialEq, Debug)]
struct Car {
  color: String,
  motor: Transmission,
  roof: bool,
  age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {Manual, SemiAuto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {New, Used}

fn car_quality (miles: u32) -> (Age, u32) {
  if miles > 0 {
    return (Age::Used, miles);
  }
  
  (Age::New, miles)
}

fn car_factory (
  order: i32,
  miles: u32
) -> Car {
  let colors = ["Blue", "Green", "Red", "Silver"];

  let mut color = order as usize;
  while color > 4 {
    color = color - 4;
  }

  let mut motor = Transmission::Manual;
  let mut roof = true;

  if order % 3 == 0 {
    motor = Transmission::Automatic;
  } else if order % 2 == 0 {
    motor = Transmission::SemiAuto;
    roof = false;
  }

  let age = car_quality(miles);

  Car {
    color: String::from(colors[(color - 1) as usize]),
    motor: motor,
    roof: roof,
    age: age
  }
}