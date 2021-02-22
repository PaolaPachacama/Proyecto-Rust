#![feature(decl_macro)]
use rocket::{self, get,post,routes};
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

extern crate serde_json;
use serde_json::Value;
use std::string::String;

//use std::fmt;
//use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct NewTodo {
    titulo: String
}

  #[get("/hello")]
  fn hola()->&'static str {
      "¡mensaje enviado desde get!"
  }
  //JSON - String 
#[post("/", data="<new_todo>")]
fn create(new_todo:Json<NewTodo>)->String{
    String::from(format!("mensaje {}",new_todo.titulo))    
 }
  
 #[post("/conver", data="<new_todo>")]
    fn enteros(new_todo:Json<NewTodo>)->String{
        let aux=&new_todo.titulo;
        let sub_strings_decimal: Vec<&str> = aux.split(',').collect();
        //println!(" valor split  {:?}",sub_strings_decimal.len());
        let sub_strings_enteros: Vec<&str> = sub_strings_decimal[0].split('.').collect();
         let mut mensaje = String::new();
         let mut mensaje_final =String::new();
         //decimales
         let decimales = sub_strings_decimal[1];
         let mut a:usize =decimales.len()-1;
        //tamaño 
         println!(" sub_string [1]=  {:?}",sub_strings_decimal[1].len());

         let mut numero_decimal = String::new();
        let mut aux_decimal=String::new();
        let mut cont=1; 
        let mut  p=0;

    let mut vector:Vec<String>= Vec::new();
    /*Decimales*/
    if sub_strings_decimal[1].len()>24 ||sub_strings_decimal[1].len()==0 {
        numero_decimal="decimales hasta 10^-24".to_string();
    }else{
        while a>=0{
            aux_decimal =decimales.chars().nth(a).unwrap().to_string() +&aux_decimal;
            if cont==3{
                cont=0;
                vector.push(aux_decimal.to_string());
                aux_decimal = String::new();                 
            }
            cont+=1;
            if a==0 {
                if cont!=1{
                    vector.push(aux_decimal.to_string());

                }
                break;
            }
         a=a-1;           
        }
        let long =vector.len();
        a=0;
        while a<long{
                if a%2==0 && a!=decimales.len()-1&& a!=0{   
                    println!("valor a  {:?}",a);
                    numero_decimal=" ".to_string() +&get_millones(p)+ " "+ &numero_decimal;
                     p+=1;  
               }else if a%2!=0{           
                  numero_decimal = " mil ".to_string()+&numero_decimal;
               }                                   
               numero_decimal =get_convert(vector[a].parse::<usize>().unwrap())+&numero_decimal;         
                a=a+1;           
        }
        numero_decimal=numero_decimal+" "+&get_indice(decimales.len());
    }
             

    /*Enteros*/        
    if sub_strings_enteros.len()>40{
         mensaje ="Números enteros hasta 10^120".to_string();
    }else{
        let mut num: usize;   
        let mut k:usize =sub_strings_enteros.len()-1;
        let mut j=0; 
        while k>=0{
            if k%2==0 && k!=sub_strings_enteros.len()-1{   
            //millones
                mensaje=" ".to_string() +&get_millones(j)+ " "+ &mensaje;
                j+=1;
            }else if k%2!=0{
                //miles
                mensaje = " mil ".to_string()+&mensaje;
            }
            mensaje =get_convert(sub_strings_enteros[k].parse::<usize>().unwrap())+&mensaje;
            if k==0{
                 break;
            }
                k=k-1;      
        }
    } 
    String::from(format!("{}",mensaje + " , "+&numero_decimal))
}

 /*Función convierte números aletras*/   
fn get_convert (num:usize)->String{
        /*cinvierte números a letras*/
        let mut numero_letras = String::new();
        if num>0 && num<10{
            String::from(format!("{}",get_unidades(num)))    
        }else if num<20{
            String::from(format!("{}",get_especiales(num)))    
        }else if num<100{
            let unid = num%10;  
            let dec = num/10;
            String::from(format!("{} y {}",get_decenas(dec-1),get_unidades(unid)))
        }else if num<1000{
            let un = num%10;
            let de = (num/10)%10;
            let ce = num/100;
            let aux=num%100;
            if num==100{
                String::from("cien")  
            }else if aux>=10 && aux<20{ //----cambios
              //  numero_letras=get_cientos(ce-1)+ " "+&get_decenas(de-1)+ " y "+ &get_unidades(un);
           //   println!("ingresa {:?}",aux);
              numero_letras=get_cientos(ce-1)+ " "+&get_especiales(aux);
                String::from(numero_letras)
            }else{
                numero_letras=get_cientos(ce-1)+ " "+&get_decenas(de-1)+ " y "+ &get_unidades(un);
                String::from(numero_letras)
            }  
        }else { 
            String::from("numeros mayores")
        }

    }

    fn get_unidades(num:usize)->String{
        let  unidades = vec!["","uno","dos", "tres","cuatro","cinco","seis", "siete","ocho","nueve"];
    //    println!("entra unidades {:?}",num);
        String::from(format!("{}",unidades[num]))    
    }

    fn get_especiales(num:usize)->String{
        let  especiales = vec!["diez","once","doce","trece","catorce","quince","deciseis","diecisiete","dieciocho","diecinueve"];
      //  println!("entra decenas {:?}",num);
        String::from(format!("{}",especiales[(num-10)]))   
    }

    fn get_decenas(num:usize)->String{
         let  decenas = vec!["diez","veinte","treinta","cuarenta","cincuenta","sesenta","setenta","ochenta","novena"];
         String::from(format!("{}",decenas[num]))   

    }

    fn get_millones(num:usize)->String{
        let  millones = vec!["millónes","billónes","trillónes","cuatrillónes","quintillónes","sextillónes","septillónes","octillónes","nonillónes","decillónes"];
        String::from(format!("{}",millones[num]))   

   }


    fn get_cientos(num:usize)->String{
        let  centenas = vec!["ciento","doscientos","trecientos","cuatrocientos","quinientos","seiscientos ","setecientos ", "ochocientos ", "novecientos "];
       // println!("entra centen {:?}",num);
    // String::from(format!("{}",centenas[num-10]))   
    String::from(format!("{}",centenas[num]))       
    }

    fn get_decimales(num:usize)->String{
        let  decimales = vec!["décimas","centésimas","milésimas","millonésimas","milmillonésimas ","billonésimas ", "milbillonésimas ", "trillonésimas","miltrillonésimas","cuatrillonésimas"];
       // println!("entra centen {:?}",num);
    // String::from(format!("{}",centenas[num-10]))   
    String::from(format!("{}",decimales[num]))       
    }

    fn get_indice(num:usize)->String{
            if num==1{
                String::from("décimas") 
            }else if num==2{
                String::from("centésimo") 
            }else if  num==3 {
                String::from("milésimo") 
            }else if  num>=4 && num<=6{
                String::from("millonésimo") 
            }else if  num>=7 && num<=9{
                String::from("milmillonésimo") 
            }else if  num>=10 && num<=12{
                String::from("billonésimo") 
            }else if  num>=13 && num<=15{
                String::from("milbillonésimo") 
            }
            else if  num>=16 && num<=18{
                String::from("trillonésimo") 
            }else if  num>=19 && num<=21{
                String::from("milmillonésimo") 
            }else if num>=22&&num<=24{
                String::from("cuatrillonésimo") 
            }else{
                String::from("numero no   válido")  
            }

    }
    
fn main() {
    rocket::ignite().mount("/", routes![hola,create,enteros]).launch();

}
