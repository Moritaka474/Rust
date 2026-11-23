// Structure

// Syntax
/*
   struct name
   {
      var: dataype,
   }

   fn main()
   {
      let instance = name
      {
         var: datatype,
      };

    println("{}",instance.var);
   }
*/

struct Employee {
    name: String,
    id: f32,
    salary: u32,
}

fn main() {
    let employee1 = Employee {
        name: String::from("Alex"),

        id: 12.45,
        salary: 50000,
    };

    println!("Name: {}", employee1.name);
    println!("ID: {}", employee1.id);
    println!("Salary: {}", employee1.salary);
}
