 fn is_a_palindrome(value:&str) -> bool{
    let name= value.to_string();
    let name_reversed = value.chars().rev().collect::<String>();   
     if name == name_reversed{
      return true;
     }else {
     panic!("isso não é um palíndromo")

     }
   
}


   fn contains_target_at(value: [i8;5], target: i8)-> bool {
    for num in value.iter(){
        if *num == target{
            return true;
    }
  }     return false;
  }




  fn contains_pars_at(value: [i8;5], target:i8) -> i8{
    let mut contagem: i8 = 0;
        for &num in value.iter() {
            if num == target {
                contagem += 1;
            }
        }
    
        if contagem > 0 {
            println!("Número que repetiu foi: {}", contagem);
            return contagem;
        } else {
            panic!("Não existe o número repetido");
        }
    }






#[cfg(test)]

mod tests{
    use crate::*;

  #[test]
  fn test_function(){
  assert!(is_a_palindrome("ala"))
  }




#[test]
fn test_contains(){
  let value = [1,2,3,4,5];
  assert!(contains_target_at(value,3))
}




#[test]
fn test_contains_pars(){
  let  list = [1,2,2,3,4];
  let target = 2;
  assert_eq!(2,contains_pars_at(list,target))

    }
}
