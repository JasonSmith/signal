mod signal;

use std::fmt;
use std::any::Any;
use std::rc::Rc;
use signal::{Event, Signal};


pub struct Shouter;

pub struct NameChangedArgs
{
   pub old: String,
   pub new: String
}

pub struct Person
{
   pub name_changed: Signal<NameChangedArgs>,
   pub name: String
}


impl Person
{
   pub fn new(first_name: &str) -> Person
   {
      Person {name_changed: Signal::new(), name: first_name.to_string()}
   }

   pub fn change_name(&mut self, new_name: &str)
   {
      let event: Event<NameChangedArgs>;

      event = Event { args: NameChangedArgs {old: self.name.to_string(), new: new_name.to_string()}};

      self.name = new_name.to_string();
      self.name_changed.call(&event);
   }
}

impl fmt::Display for Person
{
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
   {
      write!(formatter, "{}", self.name)
   }
}

impl Shouter
{
   pub fn shout(&self, event: &Event<NameChangedArgs>)
   {
      println!("Obj: Name was changed from {} to {}",
               event.args.old, event.args.new);
   }
}

fn on_name_changed(object: &Any, event: &Event<NameChangedArgs>)
{
   match object.downcast_ref::<Shouter>()
   {
      Some(shouter) =>
      {
         shouter.shout(event);
      },

      None =>
      {
         println!("Error casting name changed object to Shouter.");
      }
   }
}

fn on_name_changed_2(event: &Event<NameChangedArgs>)
{
   println!("Func: Name was changed from {} to {}",
            event.args.old, event.args.new);
}


fn main()
{
   let shouter: Rc<Shouter> = Rc::new(Shouter);
   let mut person: Person;

   person = Person::new("Jason");
   person.name_changed.connect_with_data(shouter, on_name_changed);
   person.name_changed.connect(on_name_changed_2);
   person.change_name("Shelle");
   person.name_changed.disconnect_with_data(on_name_changed);
}
