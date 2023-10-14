//Defining a module, pub is required to expose the metods to other modules

pub fn asum( x :i64, y:i64) ->i64
{
    assert!(x!=0 && y!=0);
    x+y
}