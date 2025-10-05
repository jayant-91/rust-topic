

trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

struct Sword {
    pub name: String,
    pub damage: u16,
    swing_time_ms: u16
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 1000_u16) / self.swing_time_ms * 10u16 
    }

    fn description(&self) -> String {
        format!("{}, damage: {}, swing time: {}ms", self.name, self.damage, self.swing_time_ms)
    }
}


struct Shield {
    pub name: String,
    pub armor: u16,
    pub block: u16
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor + self.block
    }

    fn description(&self) -> String {
        format!("{}, armor: {}, block: {}ms", self.name, self.armor, self.block)
    }
}


// Static dispatcher is known at compile time
// so compiler generate this function for all type those impliment the trait
// that's why it optimaize and first 
fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("I offer you: {} [{}g]", item.description(), item.price())
}

// Dynamic dispatcher is not known at compile time its only known at runtime
// The object function get at runtime is trait object
// Because compiler doesn't know at compile time it use VTable to call teh correct methods
// this is slower but also more felxible
// we can store different Sellable types in the same collection (Vec<dyn Sellable>)
// we can pass arround trait object without caring what the concrete type is
fn vendor_text_dynamic(item: &dyn Sellable) -> String {
    format!("I offer you: {} [{}g]", item.description(), item.price())
}


fn main() {
    let sword = Sword {
        name: "Sword of Cowardice".into(),
        damage: 10,
        swing_time_ms: 1500,
    };

    let shield = Shield {
        name: "Golden Barrier".into(),
        armor: 50,
        block: 35,
    };


    println!("{}", vendor_text_static(&sword));
    println!("{}", vendor_text_static(&shield));

    // Defining list of sellable item
    // For defining trait type dynamic variable we have to put "dyn" keyword before the trait type
    // this vector can only store reference of these trait objects
    // if we need to have the ownership of thesse trait objects we need to put them into boxes
    let sellagles: Vec<&dyn Sellable> = vec![&sword, &shield];

    // Looping over the loop 
    for s in sellagles {
        println!("{}", vendor_text_dynamic(s));
    }


    // if we need to have the wonership of the trait objects in the vendor vector as well
    //  we need to put them into boxes
    let owned_sellables: Vec<Box<dyn Sellable>> = vec![
        //Box::new() take teh wonership and new Sword and Shild stored in the heap
        Box::new(
            Sword{ 
                name: "Blade in teh Heap".into(), 
                damage: 55, 
                swing_time_ms: 3000 
            }
        ), 
        Box::new(
            Shield{
                name: "Shield of dynamic Memory".into(),
                armor: 130,
                block: 55,
            }
        )
    ];

    for s in &owned_sellables {
        println!("{}", vendor_text_dynamic(s.as_ref()));
    }
}
