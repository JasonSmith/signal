use std::any::{Any, TypeId};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

pub type CallbackID = usize;

pub struct Event<T>
{
   pub args: T
}

struct CallbackEntry<T>
{
   callback: Rc<RefCell<FnMut(&Event<T>)>>
}

struct DataEntry<T>
{
   target: Rc<Any>,
   callback: Rc<RefCell<FnMut(&Any, &Event<T>)>>
}


pub struct Signal<T>
{
   entries: Vec<Box<SignalEntry<T>>>
}


trait SignalEntry<T>
{
//   fn get_id() -> usize;
   fn call(&self, event: &Event<T>);
}

impl<T> SignalEntry<T> for CallbackEntry<T>
{
   fn call(&self, event: &Event<T>)
   {
      (&mut *(self.callback.borrow_mut()))(&event);
   }
}

impl<T> SignalEntry<T> for DataEntry<T>
{
   fn call(&self, event: &Event<T>)
   {
      (&mut *(self.callback.borrow_mut()))(&(*self.target), &event);
   }
}

impl<T> Signal<T>
{
   pub fn new() -> Signal<T>
   {
      Signal {entries: Vec::<Box<SignalEntry<T>>>::new()}
   }

   pub fn connect_with_data<F>(&mut self, data: Rc<Any>, callback: F)
      where F: FnMut(&Any, &Event<T>) + 'static,
            T: 'static
   {
      let entry: DataEntry<T>;
      let cell: Rc<RefCell<F>>;

      println!("Callback passed:  {:?}", callback as *mut F);

      cell = Rc::new(RefCell::new(callback));
      entry = DataEntry {target: data, callback: cell};
      self.entries.push(Box::new(entry));
   }

   pub fn connect<F>(&mut self, callback: F)
      where F: FnMut(&Event<T>) + 'static,
            T: 'static
   {
      let entry: CallbackEntry<T>;
      let cell: Rc<RefCell<F>>;

      cell = Rc::new(RefCell::new(callback));
      entry = CallbackEntry {callback: cell};
      self.entries.push(Box::new(entry));
   }

   pub fn disconnect_with_data<F>(&mut self, callback: F)
      where F: FnMut(&Any, &Event<T>) + 'static,
            T: 'static
   {
      println!("Callback passed:  {:?}", callback as *mut F);
   }

   pub fn call(&mut self, event: &Event<T>)
   {
      for entry in self.entries.iter()
      {
         entry.call(event);
      }
   }
}
