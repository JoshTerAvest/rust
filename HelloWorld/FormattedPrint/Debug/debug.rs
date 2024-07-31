fn main() {
   println!{"{} days", 31}

   println!("{0}, this is {1}. {1}, this is {0}", "Katara", "Aang");

   println!("{subject} {verb} {object}",
         object="the world",
         subject="the clever young avatar",
         verb="brings balance to");

   println!("Base 10:                  {}",     69420);
   println!("Base 2 (binary):          {:b}",   69420);
   println!("Base 8 (octal):           {:o}",   69420);
   println!("Base 16 (hexadecimal):    {:x}",   69420);

   println!("{number:>5}", number=1);

   println!("{number:0>5}", number=1);

   println!("{number:0<5}", number=1);

   println!("{number:0>width$}", number=1, width=5);

   println!("My name is {0}, {1} {0}!", "Aang", "Avatar");

   #[allow(dead_code)] // disable `dead_code` which warn against unused module
   struct Structure(i32);

   // This will not compile because `Structure` does not implement
   // fmt::Display.
   // println!("This struct `{}` won't print...", Structure(3));
   // TODO ^ Try uncommenting this line
}