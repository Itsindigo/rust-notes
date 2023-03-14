fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    // this is invalid, as the `r` reference implicitly has a lifetime which ends within the block scope.

    /**
 6 |         r = &x;
   |             ^^ borrowed value does not live long enough
 7 |     }
   |     - `x` dropped here while still borrowed
 8 |
 9 |     println!("r: {}", r);
   |                       - borrow later used here
    */
    println!("r: {}", r);
}
