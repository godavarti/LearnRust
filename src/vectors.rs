
pub fn displayvector(v:Vec<i32>)
{
      
      for x in v.iter()
   {
    println!("vector value ={}",x)
   }
}

pub fn pupulate(mut v:Vec<i32>, nofelments:i32)->Vec<i32>
{
let mut i:i32=0;
  while i<nofelments
  {
    v.push(i);
    i=i+1;
  }
  v

}

pub fn elementexists(v:Vec<i32>, element:i32)->bool
{
  if v.contains(&element)
  { true
  }
  else{
    false
  }
}