// use std::any::Any;
//
// pub trait Downcast {
//     fn as_any(&self) -> &dyn Any;
//     fn as_point_any<'a>(self: & 'a Box<Self>) -> &'a dyn Any where Self: Sized;
// }
//
// pub trait YourTrait: Downcast {
//     fn hello(&self);
// }
//
// pub trait SeeTrait: Downcast {
//     fn hello(&self);
// }
//
// pub struct SeeStruct {
//
// }
//
// impl Downcast for SeeStruct {
//     fn as_any(&self) -> &dyn Any
//         where Self: Sized
//     {
//         self
//     }
//     fn as_point_any<'a>(self: & 'a Box<Self>) -> &'a dyn Any where Self: Sized {
//         self
//     }
// }
//
// impl SeeTrait for SeeStruct {
//     fn hello(&self) {
//     }
// }
//
// struct YourStruct {
//     pub info: String,
//     pub see: Box<dyn SeeTrait>,
// }
//
// impl Downcast for YourStruct {
//     fn as_any(&self) -> &dyn Any
//         where Self: Sized
//     {
//         self
//     }
//
//     fn as_point_any<'a>(self: & 'a Box<Self>) -> &'a dyn Any where Self: Sized {
//         self
//     }
// }
//
// impl YourTrait for YourStruct {
//     fn hello(&self) {
//         println!("Hello {}", self.info);
//     }
// }
//
// #[allow(unused_variables)]
// #[test]
// fn test_type() {
//     let dyn_trait: Box<dyn YourTrait>  = Box::new(YourStruct {
//         info: "HH".to_string(),
//         see: Box::new(SeeStruct{})
//     });
//
//
//    // dyn_trait.hello(); // Hello HH
//
//     // let your_struct = dyn_trait.as_any_mut().downcast_mut::<YourStruct>().unwrap();
//     // your_struct.info = "HHHH".to_string();
//     //
//     // dyn_trait.hello(); // Hello HHHH
//
//     // match dyn_trait.as_point_any().downcast_ref::<Box<YourStruct>>() {
//     //     None => {}
//     //     Some(_) => {}
//     // }
//
//     //let xx = dyn_trait.into_any().downcast_ref::<Box<YourStruct>>().unwrap();
//
//     //let your_struct = dyn_trait.into_any().downcast::<YourStruct>().unwrap();
//
//     // get ownership
//     //let your_struct: YourStruct = *dyn_trait.into_any().downcast::<YourStruct>().unwrap();
//
//     //println!("{}", xx.info); // HHHH
// }
