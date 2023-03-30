use std::{process::Command, ops::Index};

use regex::Regex;

#[tokio::main]
async fn main() {
    let exe = tokio::fs::read_to_string("기본.한").await.unwrap();

    std::fs::write("완료.bat", han(format!("@echo off\n;{exe}\n;pause"))).unwrap();


    Command::new("./완료.bat").spawn().unwrap();
}


fn han(codes: String) -> String{
    let mut end_code = String::new();
    let re_rem = Regex::new(r"주석 (?P<code>.*)").unwrap();
    let re_print = Regex::new(r"출력\((?P<code>.*)\)").unwrap();
    let re_input = Regex::new(r"입력\((?P<code>.*)\)").unwrap();
    // let re_use_var = Regex::new(r"변수\((?P<code>.*)\)").unwrap();
    let re_use_var2 = Regex::new(r"(?P<code1>.*)변수\((?P<code2>.*)\)(?P<code3>.*)").unwrap();
    let re_set_var1 = Regex::new(r"(?P<name>.*)(.*)는(?P<code>.*)").unwrap();
    let re_set_var2 = Regex::new(r"(?P<name>.*)(.*)은(?P<code>.*)").unwrap();
    
    let re_operator_plus = Regex::new(r"(?P<var>.*)\+=(?P<code>.*)").unwrap();
    let re_operator_minus = Regex::new(r"(?P<var>.*)\-=(?P<code>.*)").unwrap();
    let re_operator_multiply = Regex::new(r"(?P<var>.*)\*=(?P<code>.*)").unwrap();
    let re_operator_divide = Regex::new(r"(?P<var>.*)/=(?P<code>.*)").unwrap();

    for code in codes.split(";"){
        let mut code = code.to_string();

        if let Option::Some(_) = re_rem.captures(&code){ continue }

        if let Option::Some(a) = re_operator_plus.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}+={}\n", &a["var"].replace(" ", ""), han(s)).as_str());
            code = "".to_string();
        }else if let Option::Some(a) = re_operator_minus.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}-={}\n", &a["var"].replace(" ", ""), han(s)).as_str());
            code = "".to_string();
        }else if let Option::Some(a) = re_operator_multiply.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}*={}\n", &a["var"].replace(" ", ""), han(s)).as_str());
            code = "".to_string();
        }else if let Option::Some(a) = re_operator_divide.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}/={}\n", &a["var"].replace(" ", ""), han(s)).as_str());
            code = "".to_string();
        }

        if let Option::Some(a) = re_use_var2.captures(&code){
            let code1 = &a["code1"];
            // let code3 = &a["code3"];
            // println!("code1: {code1:?} code3: {code3:?}");
            if code1 == ""{

                code = format!("{}%{}%{}", &a["code1"], han((&a["code2"]).to_string()), han((&a["code3"]).to_string()));
                // println!("code: {code}");
            }

        }


        
        if let Option::Some(a) = re_print.captures(&code){
            end_code.push_str(format!("echo {}\n", han((&a["code"]).to_string())).as_str());
        }
        
        else if let Option::Some(a) = re_input.captures(&code){
            end_code.push_str(format!("set /p \"{}=>\"\n", han((&a["code"]).to_string())).as_str());
        }

        else if let Option::Some(a) = re_set_var1.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}={}\n", &a["name"].replace(" ", ""), han(if &s[0..1] == " "{(&s[1..]).to_string()}else {s})).as_str());
        }
        else if let Option::Some(a) = re_set_var2.captures(&code){
            let s = (&a["code"]).to_string();
            end_code.push_str(format!("set /a {}={}\n", &a["name"].replace(" ", ""), han(if &s[0..1] == " "{(&s[1..]).to_string()}else {s})).as_str());
        }
        
        else{
            end_code.push_str(&code)
        }
        
        // if let Option::Some(a) = re_print.captures(code){
        //     end_code.push_str(format!("echo {}\n", &a["code"]).as_str());
        // }
    }
    end_code
}