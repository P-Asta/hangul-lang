use std::process::Command;

use regex::Regex;

#[tokio::main]
async fn main() {
    let exe = tokio::fs::read_to_string("기본.한").await.unwrap();


    std::fs::write("완료.bat", han(format!("@echo off\n;{exe}\n;pause"))).unwrap();
}


fn han(codes: String) -> String{
    let mut end_code = String::new();
    let re_print = Regex::new(r"출력\((?P<code>.*)\)").unwrap();
    let re_input = Regex::new(r"입력\((?P<code>.*)\)").unwrap();
    let re_var = Regex::new(r"변수\((?P<code>.*)\)").unwrap();
    let re_rem = Regex::new(r"주석 (?P<code>.*)").unwrap();

    for code in codes.split(";"){
        if let Option::Some(_) = re_rem.captures(code){
        }
        
        else if let Option::Some(a) = re_print.captures(code){
            end_code.push_str(format!("echo {}\n", han((&a["code"]).to_string())).as_str());
        }
        
        else if let Option::Some(a) = re_input.captures(code){
            end_code.push_str(format!("set /p \"{}=>\"\n", han((&a["code"]).to_string())).as_str());
        }
        
        else if let Option::Some(a) = re_var.captures(code){
            end_code.push_str(format!("%{}%", han((&a["code"]).to_string())).as_str());
        }
        
        else{
            end_code.push_str(code)
        }
        
        // if let Option::Some(a) = re_print.captures(code){
        //     end_code.push_str(format!("echo {}\n", &a["code"]).as_str());
        // }
    }
    end_code
}